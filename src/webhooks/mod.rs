use postgres;
use crate::err::HecateError;

#[derive(Serialize, Deserialize)]
pub struct WebHook {
    id: Option<i64>,
    name: String,
    actions: Vec<String>,
    url: String
}

impl WebHook {
    pub fn new(id: i64, name: String, actions: Vec<String>, url: String) -> Self {
        WebHook {
            id: Some(id),
            name: name,
            actions: actions,
            url: url
        }
    }
}

pub fn list(conn: &impl postgres::GenericConnection) -> Result<Vec<WebHook>, HecateError> {
    match conn.query("
        SELECT
            id,
            name,
            actions,
            url
        FROM
            webhooks
    ", &[]) {
        Ok(results) => {
            let mut hooks: Vec<WebHook> = Vec::with_capacity(results.len());

            for result in results.iter() {
                hooks.push(WebHook::new(result.get(0), result.get(1), result.get(2), result.get(3)));
            }

            Ok(hooks)
        },
        Err(err) => Err(HecateError::from_db(err))
    }
}

pub fn delete(conn: &impl postgres::GenericConnection, id: i64) -> Result<bool, HecateError> {
    match conn.execute("
        DELETE FROM webhooks
        WHERE id = $1
    ", &[&id]) {
        Ok(_) => Ok(true),
        Err(err) => Err(HecateError::from_db(err))
    }
}

pub fn create(conn: &impl postgres::GenericConnection, name: serde_json::Value) -> Result<WebHook, HecateError> {
    let mut webhook: WebHook = match serde_json::from_value(name) {
        Ok(webhook) => webhook,
        Err(err) => { return Err(HecateError::new(400, String::from("Invalid webhook JSON"), Some(err.to_string()))); }
    };

    match conn.query("
        INSERT INTO webhooks (name, actions, url)
            VALUES (
                $1,
                $2,
                $3
            )
            Returning id
    ", &[&webhook.name, &webhook.actions, &webhook.url]) {
        Ok(results) => {
            let id = results.get(0).get(0);

            webhook.id = Some(id);

            Ok(webhook)
        },
        Err(err) => Err(HecateError::from_db(err))
    }
}

pub fn update(conn: &impl postgres::GenericConnection, id: i64, name: serde_json::Value) -> Result<WebHook, HecateError> {
    let mut webhook: WebHook = match serde_json::from_value(name) {
        Ok(webhook) => webhook,
        Err(err) => { return Err(HecateError::new(400, String::from("Invalid webhook JSON"), Some(err.to_string()))); }
    };

    webhook.id = Some(id);

    match conn.execute("
         UPDATE webhooks
            SET
                name = $1,
                actions = $2,
                url = $3
            WHERE id = $4
    ", &[&webhook.name, &webhook.actions, &webhook.url, &id]) {
        Ok(_) => Ok(webhook),
        Err(err) => Err(HecateError::from_db(err))
    }
}
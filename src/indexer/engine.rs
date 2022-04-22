use anyhow::{anyhow, Result};
use near_lake_framework::near_indexer_primitives::StreamerMessage;
use reqwest::{Client, StatusCode};
use std::env;

pub async fn push_block_to_engine(message: StreamerMessage) -> Result<()> {

    if env::var("PUSH_ENGINE")?.parse::<bool>()? {
        return Ok(())
    }

    let json = serde_json::to_value(message)?;
    let response = Client::new()
        .post(env::var("PUSH_ENGINE_URL")?)
        .json(&json)
        .send()
        .await?;
    return if StatusCode::is_success(&response.status()) {
        Ok(())
    } else {
        Err(anyhow!(""))
    };
}

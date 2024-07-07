use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
// use cw20::{Cw20Coin, MinterResponse};
// use cw20_base::msg::InstantiateMarketingInfo;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_balances: Vec<Cw20Coin>,
    pub mint: Option<MinterResponse>,
    pub marketing: Option<InstantiateMarketingInfo>,
}

// If you need MigrateMsg, define it here. Otherwise, remove the import from contract.rs.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

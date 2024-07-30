use anchor_lang::prelude::*;

declare_id!("DBuzDQGCzxrMt8SH6pCN5STWds9cSKud9SumF2aNfPD3");

#[program]
pub mod testapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

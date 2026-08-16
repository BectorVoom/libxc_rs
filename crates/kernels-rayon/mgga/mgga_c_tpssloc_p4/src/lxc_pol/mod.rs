//! MGGA_C_TPSSLOC lxc_pol shard mgga_c_tpssloc_p4 — thin module index (no cube wrapper; the wrapper lives in the facade).
//! Parts 32..=39.

pub mod part32;
pub mod part33;
pub mod part34;
pub mod part35;
pub mod part36;
pub mod part37;
pub mod part38;
pub mod part39;

pub use part32::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8;
pub use part33::mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9;
pub use part34::mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10;
pub use part35::mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11;
pub use part36::mgga_c_tpssloc_lxc_pol_part36_v4rho3lapl_v4rho3tau_0;
pub use part37::mgga_c_tpssloc_lxc_pol_part37_v4rho3tau_1;
pub use part38::mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2;
pub use part39::mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3;

//! GGA_C_GAPLOC kernel — split into per-function files.

pub mod exc_unpol;
pub mod kxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2_v3rho3;
pub mod kxc_pol_part1_v3rho2sigma;
pub mod kxc_pol_part2_v3rhosigma2;
pub mod lxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2_v3rho3;
pub mod lxc_pol_part1_v3rho2sigma;
pub mod lxc_pol_part2_v3rhosigma2;
pub mod lxc_pol_part35_v4rhosigma3_0;
pub mod lxc_pol_part37_v4rhosigma3_2;
pub mod lxc_pol_part40_v4rhosigma3_5;
pub mod lxc_pol_part44_v4rhosigma3_9;
pub mod lxc_pol_part47_v4rhosigma3_12;
pub mod lxc_pol_part4_v4rho4;
pub mod lxc_pol_part50_v4rhosigma3_15;
pub mod lxc_pol_part54_v4rhosigma3_19;
pub mod lxc_unpol_part1_v4rho4;
pub mod lxc_unpol_part2_v4rho3sigma;
pub mod lxc_unpol_part3_v4rho2sigma2;
pub mod vxc_unpol;

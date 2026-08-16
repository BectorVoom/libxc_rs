//! MGGA_C_KCISK lxc_pol shard mgga_c_kcisk_p0 — thin module index (no cube wrapper; the wrapper lives in the facade).
//! Parts 0..=2.

pub mod part0;
pub mod part1;
pub mod part2;

pub use part0::mgga_c_kcisk_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau;
pub use part1::mgga_c_kcisk_lxc_pol_part1_v2rho2;
pub use part2::mgga_c_kcisk_lxc_pol_part2_v2rhosigma_v2rholapl_v2rhotau_v2sigma2_v2sigmalapl_v2sigmata_etc;

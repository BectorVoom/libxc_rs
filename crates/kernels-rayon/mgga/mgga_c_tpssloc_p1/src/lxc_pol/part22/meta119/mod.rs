//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta119 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk802;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk803;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk804;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk805;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk806;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk807;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk808;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk809;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta119(t1208: f64, t476: f64, t478: f64, t3036: f64, t483: f64, t3500: f64, t475: f64, t1210: f64, t121: f64, t1229: f64, t1090: f64, t248: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3502, t3503) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk802(t1208, t476, t478);
        let (t3504, t3505) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk803(t3036, t483, t3503);
        let t3506 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk804(t3500, t3505);
        let t3508 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk805(t475);
        let t3514 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk806(t1210, t3504);
        let t3515 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk807(t3500, t3514);
        let t3521 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk808(t121, t1229);
        let t3523 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk809(t1090, t248, t3521);
    (t3502, t3503, t3505, t3506, t3508, t3514, t3515, t3521, t3523)
}

//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta493 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1803;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1804;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1805;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1806;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1807;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta493(t25276: f64, t25328: f64, t858: f64, t23237: f64, t7479: f64, t6552: f64, t4119: f64, t6554: f64, t6553: f64, t23204: f64, t23164: f64, t225: f64, t7511: f64, t13042: f64, t1912: f64, t23249: f64, t23252: f64, t23254: f64, t23262: f64, t25230: f64, t25233: f64, t2597: f64, t2713: f64, t7517: f64, t855: f64, t866: f64, t25173: f64, t25196: f64, t25228: f64, t870: f64, t2752: f64, t7540: f64, t1530: f64, t776: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25329, t25330, t25338, t25339, t25341, t25342, t25343, t25345, t25346, t25348) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1803(t25276, t25328, t858, t23237, t7479, t6552, t4119, t6554, t6553, t23204, t23164, t225, t7511);
        let t25351 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1804(t13042, t1912, t23249, t23252, t23254, t23262, t25230, t25233, t25330, t25339, t25343, t25346, t25348, t2597, t2713, t7517, t855, t866);
        let (t25353, t25354) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1805(t25173, t25196, t25228, t25351, t870);
        let t25358 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1806(t2752, t7540);
        let t25365 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1807(t1530, t776);
    (t25329, t25330, t25338, t25341, t25342, t25345, t25346, t25348, t25353, t25354, t25358, t25365)
}

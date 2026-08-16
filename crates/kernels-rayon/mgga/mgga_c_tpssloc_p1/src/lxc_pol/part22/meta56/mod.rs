//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta56 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk396;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk397;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk398;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk399;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk400;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta56(t1117: f64, t1118: f64, t1099: f64, t1086: f64, t1092: f64, t432: f64, t427: f64, t1111: f64, t1103: f64, t1108: f64, t1115: f64, t435: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1119, t1121, t1122, t1124) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk396(t1117, t1118, t1099, t1086, t1092);
        let (t1127, t1128) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk397(t432);
        let t1129 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk398(t1128, t427);
        let (t1131, t1134, t1136) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk399(t1086, t1111, t1092, t1103, t1108, t1115);
        let t1137 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk400(t435);
    (t1119, t1121, t1122, t1124, t1127, t1128, t1129, t1131, t1134, t1136, t1137)
}

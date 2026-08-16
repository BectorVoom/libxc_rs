//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta60 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk395;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk396;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk397;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk398;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk399;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk400;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta60(t1136: f64, t1137: f64, t1086: f64, t1092: f64, t449: f64, t445: f64, t440: f64, t1111: f64, t1103: f64, t1108: f64, t1115: f64, t448: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1138, t1141, t1143) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk395(t1136, t1137, t1086, t1092);
        let (t1144, t1146, t1147) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk396(t1143, t449, t445);
        let t1148 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk397(t1147, t440);
        let (t1150, t1153, t1155) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk398(t1086, t1111, t1092, t1103, t1108, t1115);
        let t1156 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk399(t448);
        let t1157 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk400(t1155, t1156);
    (t1138, t1141, t1143, t1144, t1146, t1147, t1148, t1150, t1153, t1155, t1156, t1157)
}

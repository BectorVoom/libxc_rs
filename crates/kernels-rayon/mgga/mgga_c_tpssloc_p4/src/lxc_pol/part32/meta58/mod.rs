//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta58 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk387;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk388;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk389;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk390;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta58(t1102: f64, t1107: f64, t281: f64, t415: f64, t904: f64, t241: f64, t457: f64, t1090: f64, t136: f64, t1092: f64, t1103: f64, t1105: f64, t422: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1108, t1111) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk387(t1102, t1107, t281, t415, t904);
        let (t1112, t1113) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk388(t1111, t241, t457);
        let (t1114, t1115, t1117) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk389(t1090, t1113, t136, t1092, t1103, t1105, t1108, t1112);
        let t1118 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk390(t422);
    (t1108, t1111, t1112, t1113, t1114, t1115, t1117, t1118)
}

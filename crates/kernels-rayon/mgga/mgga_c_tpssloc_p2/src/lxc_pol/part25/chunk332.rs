//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 332/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk332(t1090: f64, t1230: f64, t248: f64, t1174: f64, t1195: f64, t1198: f64, t1203: f64, t1213: f64, t1218: f64, t1224: f64, t1227: f64, t488: f64) -> (f64, f64) {
    let t1232 = t248 * t1230 * t1090;
    let t1235 = t1195 - t1174 * t1198 / 288.0_f64 + t1203 * t488 / 3072.0_f64 + t1213 * t1218 / 3072.0_f64 + t1224 - t1227 * t1232 / 4608.0_f64;
    (t1232, t1235)
}

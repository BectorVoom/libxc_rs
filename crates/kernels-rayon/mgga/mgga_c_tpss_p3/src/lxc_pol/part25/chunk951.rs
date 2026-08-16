//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 951/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk951(t140: f64, t3032: f64, t4047: f64, t1098: f64, t1100: f64, t4052: f64, t4241: f64, t9561: f64, t3067: f64, t242: f64, t3090: f64, t4056: f64) -> (f64, f64, f64, f64) {
    let t12287 = t140 * t3032;
    let t12288 = t12287 * t4047;
    let t12290 = t1098 * t12288 / 324.0_f64;
    let t12291 = t140 * t1100;
    let t12292 = t12291 * t4052;
    let t12294 = t1098 * t12292 / 216.0_f64;
    let t12317 = t9561 * t4241;
    let t12319 = t3067 * t12317 / 3456.0_f64;
    let t12359 = t242 * t3090 * t4056;
    (t12290, t12294, t12319, t12359)
}

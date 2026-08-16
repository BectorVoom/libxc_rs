//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 540/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk540(t1184: f64, t3382: f64, t1190: f64, t1162: f64, t3360: f64, t1111: f64, t1181: f64, t3201: f64, t1172: f64, t2450: f64, t1024: f64, t134: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3383 = t3382 * t1184;
    let t3385 = t3382 * t1190;
    let t3391 = t3360 * t1162;
    let t3393 = t1181 * t3201 * t1111;
    let t3394 = t3391 * t3393;
    let t3396 = t2450 * t1172;
    let t3401 = t134 * t1024;
    (t3383, t3385, t3391, t3393, t3394, t3396, t3401)
}

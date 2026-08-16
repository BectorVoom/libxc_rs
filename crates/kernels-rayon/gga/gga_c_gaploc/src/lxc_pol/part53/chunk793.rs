//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 793/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk793(t40190: f64, t587: f64, t912: f64, t2464: f64, t2465: f64, t9198: f64, t29975: f64, t31119: f64, t31120: f64, t883: f64, t2482: f64, t9272: f64, t9354: f64) -> (f64, f64, f64, f64) {
    let t40192 = t587 * t912 * t40190;
    let t40196 = t587 * t2464 * t2465 * t9198;
    let t40202 = t31119 * t31120 * t883 * t29975;
    let t40208 = t9272 * t9354 * t2482;
    (t40192, t40196, t40202, t40208)
}

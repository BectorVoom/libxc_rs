//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 393/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk393(t3545: f64, t492: f64, t105: f64, t3345: f64, t3352: f64, t3519: f64, t3532: f64, t3537: f64, t3542: f64, t1016: f64) -> (f64, f64, f64) {
    let t3546 = t492 * t3545;
    let t3549 = 0.28455006635676149599e-1_f64 * t105 * t3519 + 0.28455006635676149599e-1_f64 * t105 * t3532 + 0.47425011059460249332e-2_f64 * t3345 - 0.85365019907028448797e-1_f64 * t105 * t3537 - 0.47425011059460249332e-2_f64 * t3352 + 0.56910013271352299198e-1_f64 * t105 * t3542 - 0.28455006635676149599e-1_f64 * t105 * t3546;
    let t3553 = t1016 * t1016;
    (t3546, t3549, t3553)
}

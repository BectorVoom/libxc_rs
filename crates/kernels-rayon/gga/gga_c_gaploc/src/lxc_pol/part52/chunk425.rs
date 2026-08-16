//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 425/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk425(t1437: f64, t1645: f64, t1571: f64, t528: f64, t1561: f64, t565: f64, t1559: f64, t158: f64, t120: f64, t19: f64, t196: f64, t1563: f64, t171: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4446 = t1645 * t1437;
    let t4507 = t528 * t1571;
    let t4511 = t565 * t1561;
    let t4524 = t1559 * t158;
    let t4525 = t120 * t4524;
    let t4526 = t4525 * t19;
    let t4527 = t196 * t4526;
    let t4529 = 1.0_f64 / t1563 / t171;
    (t4446, t4507, t4511, t4524, t4526, t4527, t4529)
}

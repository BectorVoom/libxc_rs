//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 672/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk672(t3649: f64, t3696: f64, t6364: f64, t6367: f64, t6370: f64, t6375: f64, t6377: f64, t6379: f64, t572: f64, t1824: f64, t564: f64, t62: f64) -> (f64, f64, f64, f64) {
    let t6382 = -0.47063e1_f64 * t6364 + 0.31375333333333333334e1_f64 * t6367 - 0.36604555555555555556e1_f64 * t6370 - 0.16068111111111111111e1_f64 * t3649 + 0.28051666666666666666e0_f64 * t6375 - 0.56103333333333333332e0_f64 * t6377 - 0.6545388888888888889e0_f64 * t6379 - 0.46308888888888888888e0_f64 * t3696;
    let t6383 = t6382 * t572;
    let t6387 = 1.0_f64 / t1824 / t564;
    let t6388 = t62 * t6387;
    (t6382, t6383, t6387, t6388)
}

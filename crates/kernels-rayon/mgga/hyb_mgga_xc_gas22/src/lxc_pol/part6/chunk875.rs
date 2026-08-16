//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 875/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk875(t2654: f64, t7339: f64, t16: f64, t3021: f64, t1033: f64, t15: f64, t221: f64, t439: f64, t12: f64, t461: f64, t2666: f64, t1040: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7340 = t2654 * t7339;
    let t7342 = t16 * t3021;
    let t7343 = t1033 * t7342;
    let t7345 = t15 * t3021;
    let t7346 = t221 * t7345;
    let t7348 = 1.0_f64/pow_3_2(t439);
    let t7349 = t7348 * t12;
    let t7350 = t7349 * t461;
    let t7352 = t2666 * t7339;
    let t7354 = t1040 * t7342;
    (t7340, t7343, t7345, t7346, t7349, t7350, t7352, t7354)
}

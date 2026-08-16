//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 771/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk771(t7373: f64, t7380: f64, t935: f64, t313: f64, t2661: f64, t7371: f64, t2672: f64, t24: f64, t2602: f64, t862: f64, t2263: f64, t864: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7381 = t7373 * t7380;
    let t7382 = t7381 * t935;
    let t7383 = t313 * t7382;
    let t7386 = t2661 * t7371;
    let t7387 = t7373 * t2672;
    let t7388 = t7387 * t935;
    let t7389 = t313 * t7388;
    let t7394 = t24 * t2602;
    let t7395 = t862 * t7394;
    let t7397 = t864 * t2263;
    (t7382, t7383, t7386, t7388, t7389, t7394, t7395, t7397)
}

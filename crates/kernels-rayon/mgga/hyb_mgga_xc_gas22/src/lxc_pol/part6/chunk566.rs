//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 566/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk566(t1110: f64, t2649: f64, t236: f64, t439: f64, t442: f64, t14: f64, t32: f64, t2212: f64, t16: f64, t1884: f64, t1033: f64, t15: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2651 = 0.11696447245269292414e1_f64 * t1110 * t2649;
    let t2654 = 1.0_f64 / t442 / t439 * t236;
    let t2655 = t32 * t14;
    let t2656 = t2655 * t2212;
    let t2657 = t2654 * t2656;
    let t2659 = t16 * t1884;
    let t2660 = t1033 * t2659;
    let t2662 = t15 * t1884;
    (t2651, t2654, t2655, t2656, t2657, t2659, t2660, t2662)
}

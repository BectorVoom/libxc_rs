//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 870/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk870(t2294: f64, t973: f64, t2300: f64, t970: f64, t972: f64, t346: f64, t2302: f64, t979: f64, t2315: f64, t7592: f64, t7529: f64, t7538: f64, t7541: f64, t7544: f64, t7547: f64, t7560: f64, t7563: f64, t7566: f64, t7596: f64, t7599: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8335 = t2294 * t973;
    let t8338 = t970 * t2300;
    let t8343 = t972 * t972;
    let t8344 = 1.0_f64 / t8343;
    let t8345 = t346 * t8344;
    let t8346 = t2302 * t979;
    let t8349 = t979 * t2315;
    let t8362 = 0.34962962962962962963e3_f64 * t7592;
    let t8363 = -0.31466666666666666667e3_f64 * t7560 + 0.15733333333333333333e3_f64 * t7563 - 0.78666666666666666666e2_f64 * t7596 - 0.47199999999999999999e3_f64 * t7566 + 0.47199999999999999999e3_f64 * t7599 - 0.14538333333333333333e4_f64 * t7529 + 0.29076666666666666666e4_f64 * t7538 - 0.14538333333333333333e4_f64 * t7541 - 0.43614999999999999999e4_f64 * t7544 + 0.43614999999999999999e4_f64 * t7547 - t8362;
    (t8335, t8338, t8343, t8344, t8345, t8346, t8349, t8363)
}

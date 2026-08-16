//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 981/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk981(t10: f64, t2595: f64, t3624: f64, t770: f64, t2638: f64, t311: f64, t330: f64, t8113: f64, t2670: f64, t935: f64, t297: f64, t7380: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10959 = t10 * t2595;
    let t10977 = t3624 * t770;
    let t10990 = t2638 * t311;
    let t10991 = t330 * t10990;
    let t11018 = t330 * t8113;
    let t11019 = t2670 * t935;
    let t11020 = t11019 * t297;
    let t11024 = t7380 * t2670;
    (t10959, t10977, t10990, t10991, t11018, t11020, t11024)
}

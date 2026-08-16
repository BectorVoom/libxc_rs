//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 304/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk304(t314: f64, t324: f64, t899: f64, t913: f64, t917: f64, t921: f64, t927: f64, t930: f64, t931: f64, t940: f64, t943: f64, t947: f64, t951: f64, t953: f64) -> f64 {
    let t956 = 0.11360101276506094136e1_f64 * t913 * t917 - 0.23181763972770020946e0_f64 * t921 * t324 + t927 + 0.28977204965962526182e-1_f64 * t930 * t931 + 0.5848048239485271795e1_f64 * t940 * t943 - 0.4030456356669135783e-1_f64 * t947 * t314 + t951 + 0.50380704458364197288e-2_f64 * t953 * t899;
    t956
}

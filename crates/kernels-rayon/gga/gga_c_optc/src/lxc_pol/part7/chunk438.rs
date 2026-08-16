//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 438/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk438(t2069: f64, t696: f64, t2073: f64, t136: f64, t162: f64, t2078: f64, t159: f64, t155: f64, t158: f64, t652: f64) -> (f64, f64, f64, f64) {
    let t2171 = t696 * t2069;
    let t2174 = t696 * t2073;
    let t2178 = t2078 * t136 * t162;
    let t2180 = 0.19984346101817798257e0_f64 * t159 * t2178;
    let t2182 = t155 * t158 * t652;
    (t2171, t2174, t2180, t2182)
}

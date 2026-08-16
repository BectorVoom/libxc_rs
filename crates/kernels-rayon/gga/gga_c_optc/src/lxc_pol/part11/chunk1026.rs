//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1026/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1026(t155: f64, t2078: f64, t2157: f64, t652: f64, t6991: f64, t156: f64, t2155: f64, t131: f64, t133: f64, t2167: f64, t6892: f64, t136: f64, t159: f64, t162: f64, t20816: f64) -> (f64, f64, f64, f64, f64) {
    let t23071 = t155 * t2157 * t2078;
    let t23077 = t155 * t6991 * t652;
    let t23095 = 1.0_f64 / t2155 / t156;
    let t23098 = t155 * t23095 * t131 * t133;
    let t23109 = t2167 * t6892;
    let t23136 = 0.10214221340929096887e2_f64 * t159 * t20816 * t136 * t162;
    (t23071, t23077, t23098, t23109, t23136)
}

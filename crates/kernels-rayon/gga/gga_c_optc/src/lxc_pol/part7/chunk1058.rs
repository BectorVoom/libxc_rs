//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1058/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1058(t2164: f64, t7048: f64, t2174: f64, t7022: f64, t7009: f64, t7037: f64, t146: f64, t622: f64, t7000: f64, t7005: f64, t155: f64, t6165: f64, t693: f64) -> (f64, f64, f64, f64, f64) {
    let t22994 = t2164 * t7048;
    let t23008 = t7022 * t2174;
    let t23010 = t7037 * t7009;
    let t23013 = t146 * t7000 * t622;
    let t23014 = t23013 * t7005;
    let t23017 = t155 * t693 * t6165;
    (t22994, t23008, t23010, t23014, t23017)
}

//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 976/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk976(t106: f64, t664: f64, t140: f64, t6917: f64, t616: f64, t645: f64, t2029: f64, t3500: f64, t3466: f64, t624: f64, t155: f64, t6990: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9804 = t106 * t664;
    let t9839 = t6917 * t140;
    let t9870 = t645 * t616;
    let t9896 = t3500 * t2029;
    let t9917 = t3466 * t624;
    let t9954 = t155 * t6990;
    (t9804, t9839, t9870, t9896, t9917, t9954)
}

//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 528/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk528(t2634: f64, t322: f64, t140: f64, t871: f64, t6: f64, t330: f64) -> (f64, f64, f64, f64) {
    let t2635 = t322 * t2634;
    let t2638 = t871 * t140;
    let t2639 = t2638 * t6;
    let t2640 = t330 * t2639;
    (t2635, t2638, t2639, t2640)
}

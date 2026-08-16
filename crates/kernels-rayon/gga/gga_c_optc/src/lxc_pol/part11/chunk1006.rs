//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1006/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1006(t6: f64, t9771: f64, t2020: f64, t6892: f64, t2029: f64, t6875: f64, t39: f64, t55: f64, t59: f64, t87: f64, t1759: f64, t1784: f64, t1790: f64) -> (f64, f64, f64, f64, f64) {
    let t22166 = t9771 * t6;
    let t22242 = t2020 * t6892;
    let t22265 = t6875 * t2029;
    let t22274 = 24.0_f64 * t39 * t55 * t59 * t87;
    let t22277 = 36.0_f64 * t1790 * t1759 * t1784;
    (t22166, t22242, t22265, t22274, t22277)
}

//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1251/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1251(t2785: f64, t7274: f64, t913: f64, t7982: f64, t8196: f64, t2663: f64, t276: f64, t308: f64, t115: f64, t282: f64, t8206: f64, t25797: f64, t2674: f64, t8134: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25821 = t913 * t7274 * t2785;
    let t25826 = t8196 * t7982;
    let t25834 = 1.0_f64 / t2663 / t308 / t276;
    let t25836 = t282 * t25834 * t115;
    let t25837 = t8206 * t25836;
    let t25843 = t8134 * t25797 * t2674;
    (t25821, t25826, t25834, t25836, t25837, t25843)
}

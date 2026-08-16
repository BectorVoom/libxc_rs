//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 586/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk586(t2844: f64, t2845: f64, t2852: f64, t2858: f64, t2862: f64, t2866: f64, t2867: f64, t2871: f64, t2874: f64, t2877: f64, t1196: f64, t1199: f64) -> (f64, f64) {
    let t2879 = t2844 + 0.12925555555555555555e1_f64 * t2845 - 0.12925555555555555555e1_f64 * t2852 + 0.38776666666666666666e1_f64 * t2858 - 0.19388333333333333333e1_f64 * t2862 + t2866 + 0.1642e-2_f64 * t2867 - 0.4105e-3_f64 * t2871 + 0.2463e-2_f64 * t2874 - 0.12315e-2_f64 * t2877;
    let t2881 = t1196 * t1199;
    (t2879, t2881)
}

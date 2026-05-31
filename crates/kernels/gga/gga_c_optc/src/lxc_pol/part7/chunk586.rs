//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 586/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk586<F: Float>(t2844: F, t2845: F, t2852: F, t2858: F, t2862: F, t2866: F, t2867: F, t2871: F, t2874: F, t2877: F, t1196: F, t1199: F) -> (F, F) {
    let t2879 = t2844 + F::cast_from(0.12925555555555555555e1_f64) * t2845 - F::cast_from(0.12925555555555555555e1_f64) * t2852 + F::cast_from(0.38776666666666666666e1_f64) * t2858 - F::cast_from(0.19388333333333333333e1_f64) * t2862 + t2866 + F::cast_from(0.1642e-2_f64) * t2867 - F::cast_from(0.4105e-3_f64) * t2871 + F::cast_from(0.2463e-2_f64) * t2874 - F::cast_from(0.12315e-2_f64) * t2877;
    let t2881 = t1196 * t1199;
    (t2879, t2881)
}

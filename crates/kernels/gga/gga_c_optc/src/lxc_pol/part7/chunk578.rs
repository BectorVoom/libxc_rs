//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 578/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk578<F: Float>(t1200: F, t1205: F, t2879: F, t2881: F, t2886: F, t2887: F, t2900: F, t485: F, t275: F, t176: F, t1186: F, t474: F, t1084: F, t411: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t2902 = -t1200 * t2900 - 2.0 * t2881 * t1205 + t2879 * t485 + 2.0 * t2886 * t2887;
    let t2903 = t2902 * t275;
    let t2905 = t176 * t2903 * sigma2;
    let t2908 = t1186 * t1186;
    let t2910 = t474 * t474;
    let t2911 = 1.0 / t2910;
    let t2915 = t1084 * t411;
    let t2916 = 1.0 / t2915;
    (t2902, t2905, t2908, t2910, t2911, t2915, t2916)
}

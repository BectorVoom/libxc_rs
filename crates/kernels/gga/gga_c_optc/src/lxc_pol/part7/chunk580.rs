//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 580/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk580<F: Float>(t1094: F, t2916: F, t2917: F, t1102: F, t2843: F, t2845: F, t2852: F, t2858: F, t2862: F, t1062: F, t1066: F, t1065: F, t398: F, t393: F, t1074: F) -> (F, F, F, F, F, F, F, F) {
    let t2919 = t2916 * t2917 * t1094;
    let t2921 = 0.11696446794910408142e1 * t1102 * t2919;
    let t2922 = 0.22831111111111111111e-1 * t2843;
    let t2927 = t2922 + 0.11415555555555555555e-1 * t2845 - 0.11415555555555555555e-1 * t2852 + 0.34246666666666666666e-1 * t2858 - 0.17123333333333333333e-1 * t2862;
    let t2930 = t1062 * t1066;
    let t2933 = t1065 * t398;
    let t2934 = 1.0 / t2933;
    let t2935 = t393 * t2934;
    let t2936 = t1074 * t1074;
    (t2919, t2921, t2927, t2930, t2933, t2934, t2935, t2936)
}

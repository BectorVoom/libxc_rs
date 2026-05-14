//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 554/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk554<F: Float>(t3703: F, t3709: F, t967: F, t696: F, t971: F, t980: F, t1023: F, t1026: F, t109: F, t138: F, t681: F, t957: F, t963: F, t683: F, t978: F, t1179: F, t282: F, t55: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3711 = t3709 * t3703 * t967;
    let t3713 = 103.89515463408878 * t696 * t3711;
    let t3714 = t971 * t980;
    let t3719 = 0.10685 * t138 * t109 * t1023 * t1026;
    let t3724 = t967 * t681;
    let t3725 = t963 * t957 * t3724;
    let t3727 = 51.94757731704439 * t696 * t3725;
    let t3729 = t978 * t957 * t683;
    let t3731 = 3.5089341735807875 * t696 * t3729;
    let t3734 = t55 * t1179 * t282;
    (t3711, t3713, t3714, t3719, t3724, t3725, t3727, t3729, t3731, t3734)
}

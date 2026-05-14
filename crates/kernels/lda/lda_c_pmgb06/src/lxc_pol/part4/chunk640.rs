//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 640/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk640<F: Float>(t3703: F, t3709: F, t967: F, t696: F, t971: F, t980: F, t1023: F, t1026: F, t109: F, t138: F, t1089: F, t27: F, t693: F, t681: F, t957: F, t963: F) -> (F, F, F, F, F, F, F, F) {
    let t3711 = t3709 * t3703 * t967;
    let t3713 = 103.89515463408878 * t696 * t3711;
    let t3714 = t971 * t980;
    let t3719 = 0.10685 * t138 * t109 * t1023 * t1026;
    let t3720 = t1089 * t27;
    let t3721 = t3720 * t693;
    let t3724 = t967 * t681;
    let t3725 = t963 * t957 * t3724;
    (t3711, t3713, t3714, t3719, t3720, t3721, t3724, t3725)
}

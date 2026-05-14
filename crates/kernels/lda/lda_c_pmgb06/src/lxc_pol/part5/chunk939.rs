//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 939/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk939<F: Float>(t16106: F, t12041: F, t16137: F, t16144: F, t16150: F, t16152: F, t495: F, t7616: F, t493: F, t499: F, t16158: F, t16161: F, t16173: F, t16178: F, t12113: F, t16181: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t19696 = 4.0 / 135.0 * t16106;
    let t19697 = 8.0 / 405.0 * t12041;
    let t19698 = 2.0 / 15.0 * t16137;
    let t19699 = 4.0 / 135.0 * t16144;
    let t19700 = 2.0 / 81.0 * t16150;
    let t19701 = 2.0 / 135.0 * t16152;
    let t19702 = t495 * t7616;
    let t19705 = t493 * t19702 * t499 / 45.0;
    let t19706 = 4.0 / 45.0 * t16158;
    let t19707 = 2.0 / 45.0 * t16161;
    let t19708 = 2.0 / 45.0 * t16173;
    let t19709 = 2.0 / 45.0 * t16178;
    let t19710 = -t19696 + t19697 + t19698 - t19699 - t19700 - t19701 + t19705 - t19706 - t19707 + t12113 + t19708 - t19709;
    let t19712 = t16181 / 15.0;
    (t19696, t19697, t19698, t19699, t19700, t19701, t19705, t19706, t19707, t19708, t19709, t19710, t19712)
}

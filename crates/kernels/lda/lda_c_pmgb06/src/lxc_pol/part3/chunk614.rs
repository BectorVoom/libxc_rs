//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 614/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk614<F: Float>(t5: F, t1090: F, t643: F, t1092: F, t638: F, t1089: F, t686: F, t248: F, t1108: F, t654: F, t1101: F, t687: F, t594: F, t1068: F, t332: F, t1074: F, t3010: F, t3115: F, t9: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t3899 = t643 * t1090;
    let t3901 = t638 * t1092;
    let t3903 = t1089 * t686;
    let t3904 = t248 * t3903;
    let t3906 = t1108 * t654;
    let t3908 = t1101 * t654;
    let t3911 = 60.0 * t1101 * t687;
    let t3912 = 1.0 / t594;
    let t3915 = t1068 * t332;
    let t3921 = piecewise3(t6, 0.0, -8.0 / 27.0 * t3912 * t3010 + 4.0 / 3.0 * t3915 * t1074 + 4.0 / 3.0 * t9 * t3115);
    (t3899, t3901, t3903, t3904, t3906, t3908, t3911, t3912, t3921)
}

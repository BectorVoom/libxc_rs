//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 939/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk939<F: Float>(t1152: F, t1200: F, t123: F, t10793: F, t199: F, t2822: F, t566: F, t4209: F, t722: F, t101: F, t4329: F, t754: F, t757: F) -> (F, F, F, F, F) {
    let t10940 = t123 * t1152 * t1200;
    let t10943 = t123 * t10793 * t199;
    let t10946 = t123 * t2822 * t566;
    let t10949 = t123 * t722 * t4209;
    let t10960 = t101 * t4329 * t754 * t757;
    (t10940, t10943, t10946, t10949, t10960)
}

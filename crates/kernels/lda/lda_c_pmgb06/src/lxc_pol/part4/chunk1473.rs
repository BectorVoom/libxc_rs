//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1473/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1473<F: Float>(t1152: F, t123: F, t2422: F, t1200: F, t125: F, t15116: F, t1808: F, t18988: F, t18995: F, t18998: F, t19004: F, t19007: F, t19017: F, t199: F, t2285: F, t2415: F, t566: F, t7117: F) -> F {
    let t19020 = t123 * t1152 * t2422;
    let t19022 = -F::cast_from(0.28298369577492777_f64) * t18988 - F::cast_from(0.031835665774679375_f64) * t123 * t2415 * t1200 + F::cast_from(0.10611888591559791_f64) * t18995 + F::cast_from(0.10611888591559791_f64) * t18998 - F::cast_from(0.1273426630987175_f64) * t123 * t2285 * t1808 + F::cast_from(0.21223777183119583_f64) * t19004 + F::cast_from(0.21223777183119583_f64) * t19007 - F::cast_from(0.031835665774679375_f64) * t123 * t125 * t15116 * t199 - F::cast_from(0.06367133154935875_f64) * t123 * t7117 * t566 - F::cast_from(0.14149184788746388_f64) * t19017 - F::cast_from(0.14149184788746388_f64) * t19020;
    t19022
}

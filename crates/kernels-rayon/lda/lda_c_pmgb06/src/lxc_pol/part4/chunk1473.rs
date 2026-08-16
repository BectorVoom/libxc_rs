//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1473/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1473(t1152: f64, t123: f64, t2422: f64, t1200: f64, t125: f64, t15116: f64, t1808: f64, t18988: f64, t18995: f64, t18998: f64, t19004: f64, t19007: f64, t19017: f64, t199: f64, t2285: f64, t2415: f64, t566: f64, t7117: f64) -> f64 {
    let t19020 = t123 * t1152 * t2422;
    let t19022 = -0.28298369577492777_f64 * t18988 - 0.031835665774679375_f64 * t123 * t2415 * t1200 + 0.10611888591559791_f64 * t18995 + 0.10611888591559791_f64 * t18998 - 0.1273426630987175_f64 * t123 * t2285 * t1808 + 0.21223777183119583_f64 * t19004 + 0.21223777183119583_f64 * t19007 - 0.031835665774679375_f64 * t123 * t125 * t15116 * t199 - 0.06367133154935875_f64 * t123 * t7117 * t566 - 0.14149184788746388_f64 * t19017 - 0.14149184788746388_f64 * t19020;
    t19022
}

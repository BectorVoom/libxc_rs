//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1167/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1167(t1069: f64, t6145: f64, t36: f64, t453: f64, t6164: f64, t1074: f64, t6159: f64, t2381: f64, t3098: f64, t1525: f64, t1438: f64, t332: f64, t5961: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15345 = t6145 * t1069;
    let t15347 = t36 * t453 * t15345;
    let t15349 = t6164 * t1069;
    let t15351 = t36 * t453 * t15349;
    let t15353 = t6159 * t1074;
    let t15355 = t36 * t453 * t15353;
    let t15358 = t3098 * t2381 * t1069;
    let t15360 = t36 * t1525 * t15358;
    let t15363 = t1438 * t5961 * t332;
    (t15345, t15347, t15349, t15351, t15353, t15355, t15358, t15360, t15363)
}

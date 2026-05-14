//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1035/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1035<F: Float>(t15345: F, t36: F, t453: F, t1069: F, t6164: F, t1074: F, t6159: F, t2381: F, t3098: F, t1525: F, t1438: F, t332: F, t5961: F, t12358: F, t12360: F, t12362: F, t12364: F, t12366: F, t12368: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15347 = t36 * t453 * t15345;
    let t15349 = t6164 * t1069;
    let t15351 = t36 * t453 * t15349;
    let t15353 = t6159 * t1074;
    let t15355 = t36 * t453 * t15353;
    let t15358 = t3098 * t2381 * t1069;
    let t15360 = t36 * t1525 * t15358;
    let t15363 = t1438 * t5961 * t332;
    let t15365 = t36 * t1525 * t15363;
    let t15367 = t6164 * t1074;
    let t15369 = t36 * t1525 * t15367;
    let t15371 = 0.010075555555555556 * t12358 - 0.0008396296296296296 * t12360 - 0.0013993827160493828 * t12362 - 0.007556666666666666 * t12364 - 0.006717037037037037 * t12366 + 0.002239012345679012 * t12368 - 0.04534 * t15347 + 0.011335 * t15351 - 0.003778333333333333 * t15355 - 0.007556666666666666 * t15360 + 0.002518888888888889 * t15365 + 0.0012594444444444445 * t15369;
    (t15347, t15349, t15351, t15353, t15355, t15358, t15360, t15363, t15365, t15367, t15369, t15371)
}

//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 921/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk921<F: Float>(t12396: F, t12397: F, t12398: F, t136: F, t1438: F, t3098: F, t441: F, t12354: F, t12356: F, t12358: F, t12360: F, t12362: F, t12364: F, t12366: F, t12369: F, t12371: F, t12374: F, t12377: F, t12380: F, t12384: F, t12387: F, t12391: F, t12393: F, t9178: F) -> (F, F, F, F) {
    let t12400 = t12396 * t12397 * t12398;
    let t12402 = t136 * t1438;
    let t12404 = t12396 * t12402 * t12398;
    let t12406 = t441 * t3098;
    let t12408 = t12396 * t12406 * t12398;
    let t12410 = -0.02770777777777778 * t12354 + 0.003778333333333333 * t12356 + 0.08312333333333333 * t12358 - 0.0012594444444444445 * t12360 - 0.002099074074074074 * t12362 - 0.011335 * t12364 - 0.005037777777777778 * t12366 + t12369 + 0.0012594444444444445 * t12371 + 0.007556666666666666 * t12374 + 0.005597530864197531 * t12377 + 0.012594444444444445 * t12380 - 0.003778333333333333 * t12384 - 0.02267 * t12387 - 0.04534 * t12391 + 0.007556666666666666 * t12393 - t9178 + 0.006297222222222222 * t12400 + 0.034005 * t12404 - 0.02267 * t12408;
    (t12400, t12404, t12408, t12410)
}

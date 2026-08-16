//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1043/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1043(t12396: f64, t12397: f64, t12398: f64, t136: f64, t1438: f64, t3098: f64, t441: f64, t12354: f64, t12356: f64, t12358: f64, t12360: f64, t12362: f64, t12364: f64, t12366: f64, t12369: f64, t12371: f64, t12374: f64, t12377: f64, t12380: f64, t12384: f64, t12387: f64, t12391: f64, t12393: f64, t9178: f64) -> (f64, f64, f64, f64) {
    let t12400 = t12396 * t12397 * t12398;
    let t12402 = t136 * t1438;
    let t12404 = t12396 * t12402 * t12398;
    let t12406 = t441 * t3098;
    let t12408 = t12396 * t12406 * t12398;
    let t12410 = -0.02770777777777778_f64 * t12354 + 0.003778333333333333_f64 * t12356 + 0.08312333333333333_f64 * t12358 - 0.0012594444444444445_f64 * t12360 - 0.002099074074074074_f64 * t12362 - 0.011335_f64 * t12364 - 0.005037777777777778_f64 * t12366 + t12369 + 0.0012594444444444445_f64 * t12371 + 0.007556666666666666_f64 * t12374 + 0.005597530864197531_f64 * t12377 + 0.012594444444444445_f64 * t12380 - 0.003778333333333333_f64 * t12384 - 0.02267_f64 * t12387 - 0.04534_f64 * t12391 + 0.007556666666666666_f64 * t12393 - t9178 + 0.006297222222222222_f64 * t12400 + 0.034005_f64 * t12404 - 0.02267_f64 * t12408;
    (t12400, t12404, t12408, t12410)
}

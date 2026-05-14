//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1238/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1238<F: Float>(t2732: F, t4042: F, t1767: F, t1770: F, t2414: F, t419: F, t4359: F, t7081: F, t2695: F, t384: F, t10599: F, t10603: F, t10606: F, t11200: F, t11574: F, t11640: F, t11645: F, t1309: F, t1312: F, t1316: F, t14567: F, t14569: F, t14571: F, t14575: F, t2180: F, t2236: F, t2308: F, t2733: F, t2741: F, t2744: F, t346: F, t388: F, t4017: F, t4045: F, t4355: F, t4358: F, t6031: F, t6032: F, t783: F) -> (F,) {
    let t18445 = t2732 * t4042;
    let t18453 = t1767 * t2414 * t419 * t1770;
    let t18474 = t4359 * t7081;
    let t18481 = t384 * t2695;
    let t18486 = 2.0 * t346 * t18445 * t4045 + 12.0 * t11574 * t2744 - 1.82185769317151e-05 * t18453 + 3.0 * t1316 * t2733 * t4017 - t346 * t2308 * t1309 * t783 + 6.0 * t2180 * t1312 * t6031 + 12.0 * t11200 * t6032 + 0.039914113367515366 * t14567 + 0.15965645347006147 * t14569 + 0.11974234010254609 * t14571 + 0.0003279343847708718 * t14575 - 2.0 * t346 * t2308 * t384 * t2236 + 12.0 * t4358 * t18474 - 12.0 * t11640 * t4355 - t346 * t11645 * t2741 + 6.0 * t2180 * t388 * t18481 + t10599 - t10603 + 0.001355981270834723 * t10606;
    (t18486,)
}

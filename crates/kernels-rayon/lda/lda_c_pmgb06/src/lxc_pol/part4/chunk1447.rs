//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1447/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1447(t2732: f64, t4042: f64, t1767: f64, t1770: f64, t2414: f64, t419: f64, t4359: f64, t7081: f64, t2695: f64, t384: f64, t10599: f64, t10603: f64, t10606: f64, t11200: f64, t11574: f64, t11640: f64, t11645: f64, t1309: f64, t1312: f64, t1316: f64, t14567: f64, t14569: f64, t14571: f64, t14575: f64, t2180: f64, t2236: f64, t2308: f64, t2733: f64, t2741: f64, t2744: f64, t346: f64, t388: f64, t4017: f64, t4045: f64, t4355: f64, t4358: f64, t6031: f64, t6032: f64, t783: f64) -> f64 {
    let t18445 = t2732 * t4042;
    let t18453 = t1767 * t2414 * t419 * t1770;
    let t18474 = t4359 * t7081;
    let t18481 = t384 * t2695;
    let t18486 = 2.0_f64 * t346 * t18445 * t4045 + 12.0_f64 * t11574 * t2744 - 1.82185769317151e-05_f64 * t18453 + 3.0_f64 * t1316 * t2733 * t4017 - t346 * t2308 * t1309 * t783 + 6.0_f64 * t2180 * t1312 * t6031 + 12.0_f64 * t11200 * t6032 + 0.039914113367515366_f64 * t14567 + 0.15965645347006147_f64 * t14569 + 0.11974234010254609_f64 * t14571 + 0.0003279343847708718_f64 * t14575 - 2.0_f64 * t346 * t2308 * t384 * t2236 + 12.0_f64 * t4358 * t18474 - 12.0_f64 * t11640 * t4355 - t346 * t11645 * t2741 + 6.0_f64 * t2180 * t388 * t18481 + t10599 - t10603 + 0.001355981270834723_f64 * t10606;
    t18486
}

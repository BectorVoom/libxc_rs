//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1192/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1192(t1972: f64, t5467: f64, t5471: f64, t2002: f64, t4767: f64, t1423: f64, t6551: f64, t4761: f64, t493: f64, t6119: f64, t4772: f64, t11860: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15734 = 2.0_f64 / 27.0_f64 * t1972 * t5467;
    let t15736 = 16.0_f64 / 81.0_f64 * t1972 * t5471;
    let t15738 = 2.0_f64 / 5.0_f64 * t2002 * t4767;
    let t15739 = t1423 * t6551;
    let t15740 = 8.0_f64 / 45.0_f64 * t15739;
    let t15743 = 2.0_f64 / 5.0_f64 * t493 * t6119 * t4761;
    let t15745 = 4.0_f64 / 45.0_f64 * t2002 * t4772;
    let t15746 = 8.0_f64 / 405.0_f64 * t11860;
    (t15734, t15736, t15738, t15740, t15743, t15745, t15746)
}

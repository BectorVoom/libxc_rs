//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1192/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1192<F: Float>(t1972: F, t5467: F, t5471: F, t2002: F, t4767: F, t1423: F, t6551: F, t4761: F, t493: F, t6119: F, t4772: F, t11860: F) -> (F, F, F, F, F, F, F) {
    let t15734 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1972 * t5467;
    let t15736 = F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t1972 * t5471;
    let t15738 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t2002 * t4767;
    let t15739 = t1423 * t6551;
    let t15740 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t15739;
    let t15743 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t493 * t6119 * t4761;
    let t15745 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t2002 * t4772;
    let t15746 = F::cast_from(8.0_f64) / F::cast_from(405.0_f64) * t11860;
    (t15734, t15736, t15738, t15740, t15743, t15745, t15746)
}

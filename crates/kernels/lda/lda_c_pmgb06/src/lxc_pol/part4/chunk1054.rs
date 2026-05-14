//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1054/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1054<F: Float>(t1972: F, t5467: F, t5471: F, t2002: F, t4767: F, t1423: F, t6551: F, t4761: F, t493: F, t6119: F, t4772: F, t11860: F, t11864: F, t11866: F, t13788: F, t2064: F, t439: F, t477: F, t822: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15734 = 2.0 / 27.0 * t1972 * t5467;
    let t15736 = 16.0 / 81.0 * t1972 * t5471;
    let t15738 = 2.0 / 5.0 * t2002 * t4767;
    let t15739 = t1423 * t6551;
    let t15740 = 8.0 / 45.0 * t15739;
    let t15743 = 2.0 / 5.0 * t493 * t6119 * t4761;
    let t15745 = 4.0 / 45.0 * t2002 * t4772;
    let t15746 = 8.0 / 405.0 * t11860;
    let t15747 = 128.0 / 405.0 * t11864;
    let t15748 = 8.0 / 405.0 * t11866;
    let t15753 = 4.0 / 5.0 * t439 * t13788 * t822 * t477 * t2064;
    (t15734, t15736, t15738, t15740, t15743, t15745, t15746, t15747, t15748, t15753)
}

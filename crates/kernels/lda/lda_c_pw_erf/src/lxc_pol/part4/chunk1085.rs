//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1085/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1085<F: Float>(t11: F, t15807: F, t3536: F, t6417: F, t945: F, t2325: F, t940: F, t9763: F, t9836: F, t325: F, t6532: F, t1125: F, t4: F, t56: F) -> (F, F, F, F, F, F, F) {
    let t15809 = t11 * t3536 * t15807;
    let t15811 = t6417 * t945;
    let t15813 = t11 * t3536 * t15811;
    let t15816 = t9763 * t2325 * t940;
    let t15818 = t11 * t9836 * t15816;
    let t15820 = t325 * t6532;
    let t15823 = t4 * t1125 * t56;
    (t15809, t15811, t15813, t15816, t15818, t15820, t15823)
}

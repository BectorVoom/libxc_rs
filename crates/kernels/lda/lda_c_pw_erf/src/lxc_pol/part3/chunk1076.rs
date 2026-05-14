//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1076/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1076<F: Float>(t14650: F, t5607: F, t2: F, t39: F, t756: F, t8901: F, t102: F, t436: F, t1568: F, t1872: F, t5651: F, t1: F, t1664: F, t322: F, t415: F, t767: F) -> (F, F, F, F, F, F, F, F) {
    let t14651 = t5607 * t14650;
    let t14652 = 3.8973666666666666 * t14651;
    let t14654 = t756 * t2 * t39;
    let t14655 = t8901 * t14654;
    let t14656 = 1.9486833333333333 * t14655;
    let t14657 = t102 * t436;
    let t14658 = t1872 * t1568;
    let t14660 = 52.61445 * t14657 * t14658;
    let t14661 = t5651 * t14658;
    let t14666 = t1664 * t1 * t322;
    let t14667 = t415 * t767 * t14666;
    (t14652, t14654, t14656, t14658, t14660, t14661, t14666, t14667)
}

//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1042/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1042<F: Float>(t14646: F, t5607: F, t4: F, t411: F, t474: F, t2: F, t39: F, t756: F, t8901: F, t102: F, t436: F, t1: F, t1664: F, t322: F, t415: F, t767: F) -> (F, F, F, F, F, F, F, F) {
    let t14647 = t5607 * t14646;
    let t14650 = t4 * t474 * t411;
    let t14651 = t5607 * t14650;
    let t14654 = t756 * t2 * t39;
    let t14655 = t8901 * t14654;
    let t14657 = t102 * t436;
    let t14666 = t1664 * t1 * t322;
    let t14667 = t415 * t767 * t14666;
    (t14647, t14650, t14651, t14654, t14655, t14657, t14666, t14667)
}

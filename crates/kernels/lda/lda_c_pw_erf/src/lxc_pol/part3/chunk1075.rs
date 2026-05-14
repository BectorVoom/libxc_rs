//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1075/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1075<F: Float>(t10: F, t128: F, t14632: F, t21: F, t411: F, t635: F, t1652: F, t763: F, t1844: F, t415: F, t5594: F, t156: F, t1568: F, t4: F, t5607: F, t474: F) -> (F, F, F, F, F, F, F) {
    let t14634 = t10 * t128 * t14632;
    let t14639 = t21 * t635 * t411;
    let t14640 = t1652 * t763 * t14639;
    let t14641 = 1.9486833333333333 * t14640;
    let t14643 = t415 * t1844 * t5594;
    let t14644 = 5.84605 * t14643;
    let t14646 = t4 * t156 * t1568;
    let t14647 = t5607 * t14646;
    let t14648 = 2.923025 * t14647;
    let t14650 = t4 * t474 * t411;
    (t14634, t14639, t14641, t14644, t14646, t14648, t14650)
}

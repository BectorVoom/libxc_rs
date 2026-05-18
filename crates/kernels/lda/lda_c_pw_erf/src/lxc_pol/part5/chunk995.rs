//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 995/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk995<F: Float>(t1055: F, t5967: F, t402: F, t6011: F, t75: F, t1051: F, t390: F, t40: F, t19: F, t729: F, t7307: F, t734: F) -> (F, F, F, F, F) {
    let t15341 = t5967 * t1055;
    let t15344 = t6011 * t75 * t402;
    let t15346 = t5967 * t1051;
    let t15349 = t40 * t6011 * t390;
    let t15413 = t7307 * t729 * t19 * t734;
    (t15341, t15344, t15346, t15349, t15413)
}

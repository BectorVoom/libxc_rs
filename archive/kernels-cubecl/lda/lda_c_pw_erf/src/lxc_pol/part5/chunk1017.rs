//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1017/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1017<F: Float>(t4794: F, t571: F, t6379: F, t12064: F, t6730: F, t10011: F, t6740: F, t108: F, t267: F, t794: F, t2471: F, t4489: F) -> (F, F, F, F, F) {
    let t16768 = t571 * t4794 * t6379;
    let t16819 = t12064 * t6730;
    let t16829 = t10011 * t6740;
    let t16858 = t794 * t108 * t267;
    let t16863 = t4489 * t2471;
    (t16768, t16819, t16829, t16858, t16863)
}

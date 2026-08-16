//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 553/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk553<F: Float>(t171: F, t2853: F, t1113: F, t169: F, t632: F, t1143: F, t703: F, t161: F, t2872: F, t1: F, t1128: F) -> (F, F, F, F, F) {
    let t2898 = t171 * t2853;
    let t2903 = t169 * t1113 * t632;
    let t2906 = t169 * t703 * t1143;
    let t2908 = t2872 * t161;
    let t2910 = t1128 * t1;
    (t2898, t2903, t2906, t2908, t2910)
}

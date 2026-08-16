//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1253/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1253<F: Float>(t1138: F, t2817: F, t2820: F, t780: F, t153: F, t474: F, t5718: F, t168: F, t2782: F, t861: F, t1125: F, t1891: F) -> (F, F, F, F) {
    let t14911 = t2817 * t780 * t1138 * t2820;
    let t14921 = t153 * t474 * t5718;
    let t14925 = t168 * t2782 * t861;
    let t14932 = t153 * t1125 * t1891;
    (t14911, t14921, t14925, t14932)
}

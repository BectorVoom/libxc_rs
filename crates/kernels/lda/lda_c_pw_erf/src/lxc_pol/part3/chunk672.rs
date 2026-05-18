//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 672/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk672<F: Float>(t3440: F, t3568: F, t3721: F, t3791: F, t3858: F, t3924: F, t3989: F, t4076: F, t1210: F, t168: F, t671: F, t1534: F, t635: F) -> (F, F, F) {
    let t4079 = t3440 + t3568 + t3721 + t3791 + t3858 + t3924 + t3989 + t4076;
    let t4084 = t168 * t1210 * t671;
    let t4087 = t168 * t635 * t1534;
    (t4079, t4084, t4087)
}

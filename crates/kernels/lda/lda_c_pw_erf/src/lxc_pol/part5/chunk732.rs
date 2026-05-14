//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 732/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk732<F: Float>(t6746: F, t6936: F, t7238: F, t7239: F, t7242: F, t7248: F, t7251: F, t7252: F, t7254: F, t7258: F, t7263: F, t7269: F, t7270: F, t7275: F, t7277: F, t7283: F) -> (F,) {
    let t7287 = t7238 + t7239 + t7242 + t7248 + t7251 + t7252 + t7254 + t7258 + t7263 + t6746 + t7269 + t7270 + t7275 + t6936 + t7277 + t7283;
    (t7287,)
}

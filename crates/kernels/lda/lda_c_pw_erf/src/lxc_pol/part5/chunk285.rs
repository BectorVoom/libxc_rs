//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 285/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk285<F: Float>(t155: F, t364: F, t363: F, t67: F, t62: F, t370: F) -> (F, F, F, F, F) {
    let t966 = t155 * t364;
    let t970 = t363 * t67;
    let t971 = 1.0 / t970;
    let t972 = t62 * t971;
    let t973 = t370 * t370;
    (t966, t970, t971, t972, t973)
}

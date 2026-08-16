//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 933/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk933<F: Float>(t1453: F, t3783: F, t519: F, t1458: F, t155: F, t1461: F, t3723: F, t3883: F, t1446: F, t3880: F, t3884: F, t3788: F, t3794: F) -> (F, F, F, F, F, F, F) {
    let t10311 = t519 * t3783 * t1453;
    let t10313 = t155 * t1458;
    let t10315 = t519 * t10313 * t1461;
    let t10318 = t519 * t3883 * t3723;
    let t10320 = t1446 * t3880;
    let t10322 = t1446 * t3884;
    let t10326 = t3794 * t3788;
    (t10311, t10313, t10315, t10318, t10320, t10322, t10326)
}

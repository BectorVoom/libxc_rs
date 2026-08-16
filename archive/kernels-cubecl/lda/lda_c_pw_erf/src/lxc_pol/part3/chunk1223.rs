//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1223/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1223<F: Float>(t11366: F, t11368: F, t11370: F, t11372: F, t11374: F, t11376: F, t11378: F, t11379: F, t11380: F, t11381: F, t11382: F, t8285: F, t8290: F, t8296: F, t8300: F, t8309: F, t8356: F) -> F {
    let t14418 = t8285 + t11366 + t8290 + t11368 - t8296 - t11370 - t11372 - t11374 - t8300 - t11376 + t11378 - t11379 + t8309 - t11380 - t11381 - t8356 + t11382;
    t14418
}

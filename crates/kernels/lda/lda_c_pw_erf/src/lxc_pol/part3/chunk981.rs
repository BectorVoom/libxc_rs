//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 981/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk981<F: Float>(t8357: F, t8375: F, t11368: F, t11370: F, t11372: F, t11374: F, t11376: F, t11378: F, t11379: F, t11380: F, t11381: F, t8296: F, t8300: F, t8301: F, t8309: F, t8356: F, t8368: F, t8373: F, t8382: F) -> (F, F, F) {
    let t11382 = F::new(8.0) * t8357;
    let t11383 = F::new(0.0001831155503675316) * t8375;
    let t11384 = t11368 - t8296 - t11370 - t11372 - t11374 - t8300 - t11376 - F::new(5.476843845342223) * t8301 + t11378 - t11379 + t8309 - t11380 - t11381 - t8356 + t11382 - t8368 - t8373 - t11383 - t8382;
    (t11382, t11383, t11384)
}

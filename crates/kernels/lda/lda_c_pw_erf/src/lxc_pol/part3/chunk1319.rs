//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1319/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1319<F: Float>(t108: F, t14260: F, t14262: F, t14264: F, t14271: F, t14275: F, t14278: F, t14283: F, t14285: F, t14287: F, t14289: F, t14291: F, t14293: F, t15198: F, t267: F) -> F {
    let t15202 = t14260 + t14262 + t14264 + t14271 - t14275 - t14278 + t14283 + t14285 + t14287 - t14289 + t14291 + t14293 - t15198 * t108 * t267 / F::cast_from(15.0_f64);
    t15202
}

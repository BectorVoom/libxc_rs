//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1137/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1137<F: Float>(t2171: F, t3803: F, t13282: F, t13287: F, t13289: F, t13293: F, t13297: F, t13299: F, t13302: F, t13304: F, t13307: F, t13310: F, t13315: F, t13317: F) -> (F, F) {
    let t13318 = t2171 * t3803;
    let t13319 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t13318;
    let t13320 = t13282 + t13287 + t13289 + t13293 - t13297 + t13299 + t13302 + t13304 + t13307 - t13310 + t13315 - t13317 - t13319;
    (t13319, t13320)
}

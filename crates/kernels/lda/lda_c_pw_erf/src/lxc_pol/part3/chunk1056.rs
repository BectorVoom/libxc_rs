//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1056/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1056<F: Float>(t12373: F, t4488: F, t4501: F, t12337: F, t12339: F, t12341: F, t12345: F, t12348: F, t12351: F, t12355: F, t12357: F, t12361: F, t12367: F, t12369: F, t12372: F) -> (F, F) {
    let t12376 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t4488 * t4501 * t12373;
    let t12377 = t12337 + t12339 + t12341 + t12345 + t12348 + t12351 + t12355 + t12357 + t12361 + t12367 - t12369 - t12372 - t12376;
    (t12376, t12377)
}

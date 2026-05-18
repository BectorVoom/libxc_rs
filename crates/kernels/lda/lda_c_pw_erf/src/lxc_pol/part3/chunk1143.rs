//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1143/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1143<F: Float>(t13389: F, t2127: F, t3455: F, t10278: F, t10286: F, t13364: F, t13367: F, t13371: F, t13373: F, t13376: F, t13377: F, t13380: F, t13381: F, t13387: F) -> (F, F, F) {
    let t13390 = F::new(32.0) / F::new(45.0) * t13389;
    let t13391 = t3455 * t2127;
    let t13392 = F::new(8.0) / F::new(15.0) * t13391;
    let t13393 = -t13364 + t13367 - t13371 - t13373 + t13376 + F::new(4.0) * t13377 + t13380 + F::new(4.0) * t13381 + F::new(4.0) * t10278 + t10286 - t13387 + t13390 + t13392;
    (t13390, t13392, t13393)
}

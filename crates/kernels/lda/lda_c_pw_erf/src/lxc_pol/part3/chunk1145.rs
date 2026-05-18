//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1145/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1145<F: Float>(t10311: F, t10315: F, t10318: F, t10320: F, t10322: F, t13396: F, t13398: F, t13400: F, t13402: F, t13403: F, t13405: F, t13407: F, t13408: F) -> (F, F, F, F, F, F) {
    let t13409 = F::new(8.0) / F::new(135.0) * t10311;
    let t13410 = F::new(8.0) / F::new(81.0) * t10315;
    let t13411 = F::new(16.0) / F::new(27.0) * t10318;
    let t13412 = F::new(8.0) / F::new(45.0) * t10320;
    let t13413 = F::new(8.0) / F::new(27.0) * t10322;
    let t13414 = t13396 + t13398 + t13400 + t13402 + t13403 - t13405 + t13407 - t13408 - t13409 - t13410 - t13411 + t13412 + t13413;
    (t13409, t13410, t13411, t13412, t13413, t13414)
}

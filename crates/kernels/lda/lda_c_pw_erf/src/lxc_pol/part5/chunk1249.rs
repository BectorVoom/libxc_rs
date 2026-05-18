//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1249/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1249<F: Float>(t2072: F, t22396: F, t13930: F, t22372: F, t22374: F, t22376: F, t22378: F, t22380: F, t22382: F, t22384: F, t22386: F, t22388: F, t22391: F, t22392: F, t22394: F) -> (F, F) {
    let t22398 = F::new(8.0) / F::new(5.0) * t22396 * t2072;
    let t22399 = t22372 - t22374 + t22376 - t22378 - t22380 - t22382 - t22384 - t22386 - t22388 - t13930 - t22391 - t22392 - t22394 - t22398;
    (t22398, t22399)
}

//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1143/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1143<F: Float>(t13916: F, t13917: F, t22332: F, t22334: F, t22338: F, t22341: F, t22343: F, t22345: F, t22350: F, t22352: F, t22354: F, t22358: F, t22361: F, t13919: F, t13925: F, t22362: F, t22363: F, t22367: F, t22369: F, t22372: F, t22374: F, t22376: F, t22378: F, t22380: F, t22382: F, t22384: F) -> (F, F) {
    let t23304 = t22332 - t22334 + t22338 + t22341 + t22343 + t22345 + t22350 + t22352 + t22354 + t22358 + t22361 + t13916 + 0.6492624817418906 * t13917;
    let t23307 = -0.2885611029963958 * t13919 - t13925 - t22362 - t22363 - t22367 + t22369 + t22372 - t22374 + t22376 - t22378 - t22380 - t22382 - t22384;
    (t23304, t23307)
}

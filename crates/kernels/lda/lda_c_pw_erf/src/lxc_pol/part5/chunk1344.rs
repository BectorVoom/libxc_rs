//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1344/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1344<F: Float>(t13916: F, t13917: F, t22332: F, t22334: F, t22338: F, t22341: F, t22343: F, t22345: F, t22350: F, t22352: F, t22354: F, t22358: F, t22361: F) -> F {
    let t23304 = t22332 - t22334 + t22338 + t22341 + t22343 + t22345 + t22350 + t22352 + t22354 + t22358 + t22361 + t13916 + F::new(0.6492624817418906) * t13917;
    t23304
}

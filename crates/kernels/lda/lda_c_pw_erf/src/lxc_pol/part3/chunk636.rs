//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 636/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk636<F: Float>(t3439: F, t3442: F, t3444: F, t3447: F, t3449: F, t3451: F, t3453: F, t3457: F, t3459: F, t3461: F, t3463: F, t3468: F, t3549: F, t3552: F, t3555: F, t3558: F, t3560: F) -> (F,) {
    let t4177 = t3439 + t3442 + t3444 + t3447 - t3449 + t3451 - t3453 + t3457 - t3459 - t3461 + t3463 + t3468 - t3549 - t3552 + t3555 - t3558 - t3560;
    (t4177,)
}

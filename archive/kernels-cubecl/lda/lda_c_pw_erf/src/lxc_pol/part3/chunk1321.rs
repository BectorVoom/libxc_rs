//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1321/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1321<F: Float>(t14340: F, t14344: F, t14347: F, t14350: F, t14352: F, t14353: F, t14354: F, t14355: F, t14357: F, t14359: F, t14361: F, t14363: F, t14366: F, t14368: F) -> F {
    let t15207 = -t14340 - t14344 - t14347 + t14350 - t14352 - t14353 + t14354 + t14355 - t14357 - t14359 - t14361 + t14363 + t14366 - t14368;
    t15207
}

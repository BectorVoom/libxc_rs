//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1311/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1311<F: Float>(t17363: F, t17364: F, t17365: F, t17368: F, t17369: F, t17370: F, t17371: F, t17372: F, t17373: F, t17374: F, t17375: F, t17378: F, t17381: F, t17385: F, t17387: F, t17389: F, t17392: F) -> (F,) {
    let t19242 = -t17363 + t17364 + t17365 + t17368 + t17369 + t17370 + t17371 - t17372 + t17373 - t17374 - t17375 - t17378 + t17381 + t17385 - t17387 - t17389 - t17392;
    (t19242,)
}

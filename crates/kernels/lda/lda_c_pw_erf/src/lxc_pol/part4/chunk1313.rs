//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1313/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1313<F: Float>(t646: F, t7045: F, t15060: F, t15062: F, t17435: F, t17437: F, t17444: F, t17445: F, t17446: F, t17448: F, t17450: F, t17452: F, t17454: F, t17457: F, t17459: F, t17462: F, t17464: F, t17465: F) -> (F,) {
    let t19249 = t7045 * t646;
    let t19253 = t17435 - t17437 + 0.033245444444444446 * t19249 + t17444 + t17445 + t17446 - t17448 + t17450 + t17452 + t17454 - 0.027012345679012346 * t15060 + 4.0 / 9.0 * t15062 - t17457 - t17459 - t17462 + t17464 - t17465;
    (t19253,)
}

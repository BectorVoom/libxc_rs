//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 820/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk820<F: Float>(t1203: F, t1590: F, t1191: F, t163: F, t169: F, t616: F, t299: F, t4239: F, t4026: F, t568: F, t185: F, t3678: F, t514: F, t196: F, t3674: F, t211: F, t3656: F) -> (F, F, F, F, F, F, F) {
    let t9207 = t1203 * t1590;
    let t9211 = t169 * t1191 * t616 * t163;
    let t9215 = t169 * t299 * t4239 * t163;
    let t9217 = t4026 * t568;
    let t9220 = t185 * t514 * t3678;
    let t9223 = 1.0 / t3674 / t196;
    let t9231 = t211 * t514 * t3656;
    (t9207, t9211, t9215, t9217, t9220, t9223, t9231)
}

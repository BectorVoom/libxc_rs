//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1209/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1209<F: Float>(t14255: F, t219: F, t9408: F, t13598: F, t571: F, t4753: F, t4781: F, t3416: F, t14230: F, t14233: F, t14236: F, t14239: F, t14243: F, t14246: F, t14248: F, t14250: F, t14252: F) -> (F, F, F, F, F) {
    let t14256 = F::new(8.0) / F::new(81.0) * t14255;
    let t14257 = t9408 * t219;
    let t14260 = F::new(352.0) / F::new(243.0) * t571 * t14257 * t13598;
    let t14262 = F::new(8.0) / F::new(15.0) * t4753 * t4781;
    let t14264 = F::new(8.0) / F::new(15.0) * t3416 * t4781;
    let t14265 = t14230 + t14233 - t14236 - t14239 + t14243 + t14246 + t14248 + t14250 - t14252 - t14256 + t14260 + t14262 + t14264;
    (t14256, t14260, t14262, t14264, t14265)
}

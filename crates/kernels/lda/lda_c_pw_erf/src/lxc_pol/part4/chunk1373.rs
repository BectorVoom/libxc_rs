//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1373/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1373<F: Float>(t11286: F, t3169: F, t3177: F, t3179: F, t3181: F, t3183: F, t3185: F, t3187: F, t3192: F, t5710: F, t6072: F, t11273: F, t11274: F, t11275: F, t11276: F, t11277: F, t11282: F, t19908: F, t19922: F, t19935: F, t3158: F, t6065: F, t6067: F, t6070: F) -> (F,) {
    let t19950 = 2.0 * t6072 - 48.0 * t3169 - t5710 + t11286 + 64.0 * t3177 + 120.0 * t3179 - 16.0 * t3181 - 48.0 * t3183 - 8.0 * t3185 - 8.0 * t3187 + 2.0 * t3192;
    let tv4rho42 = t19908 + t19922 + t19935 - t11273 + t11274 + t11275 - t11276 + t11277 + 8.0 * t6065 + 192.0 * t3158 - 8.0 * t6067 + 2.0 * t6070 - t11282 + t19950;
    (tv4rho42,)
}

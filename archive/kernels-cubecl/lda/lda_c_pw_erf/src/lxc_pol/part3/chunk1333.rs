//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1333/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1333<F: Float>(t3156: F, t3161: F, t3173: F, t11272: F, t11273: F, t11274: F, t11275: F, t11276: F, t11277: F, t11282: F, t11286: F, t3172: F, t3178: F, t3179: F, t3182: F, t3183: F, t3187: F, t3190: F, t3192: F, t8134: F) -> F {
    let t15321 = F::cast_from(48.0_f64) * t3156;
    let t15322 = F::cast_from(480.0_f64) * t3161;
    let t15323 = F::cast_from(192.0_f64) * t3173;
    let t15328 = t8134 + t11272 - t11273 + t11274 + t11275 - t11276 + t11277 - t15321 + t15322 - t11282 - t3172 - t15323 + t11286 - t3178 + F::cast_from(180.0_f64) * t3179 + t3182 - F::cast_from(72.0_f64) * t3183 - F::cast_from(24.0_f64) * t3187 + t3190 + F::cast_from(6.0_f64) * t3192;
    t15328
}

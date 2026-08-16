//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1333/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1333(t3156: f64, t3161: f64, t3173: f64, t11272: f64, t11273: f64, t11274: f64, t11275: f64, t11276: f64, t11277: f64, t11282: f64, t11286: f64, t3172: f64, t3178: f64, t3179: f64, t3182: f64, t3183: f64, t3187: f64, t3190: f64, t3192: f64, t8134: f64) -> f64 {
    let t15321 = 48.0_f64 * t3156;
    let t15322 = 480.0_f64 * t3161;
    let t15323 = 192.0_f64 * t3173;
    let t15328 = t8134 + t11272 - t11273 + t11274 + t11275 - t11276 + t11277 - t15321 + t15322 - t11282 - t3172 - t15323 + t11286 - t3178 + 180.0_f64 * t3179 + t3182 - 72.0_f64 * t3183 - 24.0_f64 * t3187 + t3190 + 6.0_f64 * t3192;
    t15328
}

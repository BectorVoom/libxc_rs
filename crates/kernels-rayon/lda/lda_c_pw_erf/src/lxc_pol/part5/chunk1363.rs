//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1363/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1363(t11272: f64, t11273: f64, t11274: f64, t11275: f64, t11276: f64, t11277: f64, t11282: f64, t11286: f64, t15321: f64, t15322: f64, t15323: f64, t3172: f64, t3178: f64, t3180: f64, t3182: f64, t3184: f64, t6067: f64, t6070: f64, t6072: f64, t7384: f64) -> f64 {
    let t23372 = t11272 - t11273 + t11274 + t11275 - t11276 + t11277 + t15321 - 24.0_f64 * t6067 + 3.0_f64 * t6070 - t15322 - t11282 + 6.0_f64 * t6072 + t7384 + t3172 + t15323 + t11286 + t3178 + t3180 - t3182 - t3184;
    t23372
}

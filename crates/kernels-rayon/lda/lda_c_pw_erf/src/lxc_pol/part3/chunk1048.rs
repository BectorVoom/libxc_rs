//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1048/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1048(t1472: f64, t4810: f64, t4813: f64, t1948: f64, t2973: f64, t1319: f64, t571: f64, t12252: f64, t12257: f64, t12259: f64, t12261: f64, t12263: f64, t12267: f64, t12271: f64, t12273: f64, t12275: f64, t12277: f64) -> (f64, f64, f64, f64, f64) {
    let t12279 = 8.0_f64 / 3.0_f64 * t1472 * t4810;
    let t12281 = 32.0_f64 / 15.0_f64 * t1472 * t4813;
    let t12282 = t1948 * t2973;
    let t12285 = 8.0_f64 / 45.0_f64 * t571 * t1319 * t12282;
    let t12286 = t12252 - t12257 + t12259 + t12261 + t12263 + t12267 + t12271 + t12273 - t12275 - t12277 - t12279 + t12281 - t12285;
    (t12279, t12281, t12282, t12285, t12286)
}

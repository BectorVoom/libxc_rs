//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1212/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1212(t13264: f64, t13266: f64, t13268: f64, t13270: f64, t13272: f64, t13277: f64, t13279: f64, t13283: f64, t13284: f64, t13285: f64, t13286: f64, t10134: f64, t13287: f64, t13288: f64, t13289: f64, t13290: f64, t13292: f64, t13295: f64, t13299: f64, t13303: f64, t13307: f64, t13311: f64, t13313: f64) -> (f64, f64) {
    let t14420 = t13264 + t13266 + t13268 + t13270 + t13272 - t13277 - t13279 - t13283 - t13284 - t13285 + t13286;
    let t14421 = t13287 - t13288 + t13289 + t13290 - t13292 - t13295 + t13299 - t10134 + t13303 - t13307 - t13311 + t13313;
    (t14420, t14421)
}

//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1122/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1122(t10137: f64, t10156: f64, t10134: f64, t13295: f64, t13299: f64, t13303: f64, t13307: f64, t13311: f64, t13313: f64, t13315: f64, t13318: f64, t13322: f64) -> (f64, f64, f64) {
    let t13323 = 2.0_f64 / 45.0_f64 * t10137;
    let t13324 = 4.0_f64 / 45.0_f64 * t10156;
    let t13325 = -t13295 + t13299 - t10134 + t13303 - t13307 - t13311 + t13313 + t13315 - t13318 + t13322 - t13323 - t13324;
    (t13323, t13324, t13325)
}

//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 606/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk606(t103: f64, t3336: f64, t3338: f64, t3341: f64, t3344: f64, t3347: f64, t3350: f64, t3352: f64, t3354: f64, t3359: f64, t3362: f64, t3368: f64) -> f64 {
    let t3369 = -0.02666666666666667_f64 * t3336 + 0.013333333333333334_f64 * t103 * t3338 - 0.006666666666666667_f64 * t103 * t3341 - 0.04_f64 * t103 * t3344 + 0.04_f64 * t103 * t3347 - 0.022222222222222223_f64 * t3350 + 0.013333333333333334_f64 * t3352 + 0.0044444444444444444_f64 * t3354 - 0.002962962962962963_f64 * t103 * t3359 - 0.006666666666666667_f64 * t103 * t3362 - t3368;
    t3369
}

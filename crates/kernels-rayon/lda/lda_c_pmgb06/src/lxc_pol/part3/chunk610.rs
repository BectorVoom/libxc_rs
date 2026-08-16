//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 610/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk610(t3080: f64, t1619: f64, t3099: f64, t3104: f64, t3108: f64, t473: f64, t2970: f64, t103: f64, t3396: f64, t3398: f64, t3400: f64, t3405: f64, t3408: f64, t3413: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3414 = 0.11197407407407407_f64 * t3080;
    let t3415 = t1619 * t3099;
    let t3418 = t1619 * t3104;
    let t3421 = t473 * t3108;
    let t3424 = t473 * t2970;
    let t3427 = -0.022222222222222223_f64 * t3396 + 0.013333333333333334_f64 * t3398 + 0.0044444444444444444_f64 * t3400 - 0.002962962962962963_f64 * t103 * t3405 - 0.006666666666666667_f64 * t103 * t3408 - t3413 - t3414 + 0.013333333333333334_f64 * t103 * t3415 - 0.006666666666666667_f64 * t103 * t3418 - 0.04_f64 * t103 * t3421 + 0.04_f64 * t103 * t3424;
    (t3414, t3415, t3418, t3421, t3424, t3427)
}

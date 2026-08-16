//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 813/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk813(t1349: f64, t7418: f64, t11: f64, t557: f64, t7422: f64, t7426: f64, t3633: f64, t7408: f64, t7404: f64, t6638: f64, t6649: f64, t6657: f64, t6793: f64, t6795: f64, t6797: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7434 = t1349 * t7418;
    let t7435 = t11 * t7434;
    let t7437 = t557 * t7422;
    let t7438 = t11 * t7437;
    let t7440 = t557 * t7426;
    let t7441 = t11 * t7440;
    let t7449 = t3633 * t7408;
    let t7450 = t11 * t7449;
    let t7452 = t557 * t7404;
    let t7453 = t11 * t7452;
    let t7455 = -0.07198333333333333_f64 * t7435 - 0.21595_f64 * t7438 + 0.21595_f64 * t7441 + 0.013333333333333334_f64 * t6793 + 0.0044444444444444444_f64 * t6795 - 0.02666666666666667_f64 * t6797 - 0.07198333333333333_f64 * t6649 + 0.035991666666666665_f64 * t6657 + 0.023994444444444443_f64 * t6638 - 0.03999074074074074_f64 * t7450 - 0.035991666666666665_f64 * t7453;
    (t7434, t7435, t7437, t7438, t7440, t7441, t7449, t7450, t7452, t7453, t7455)
}

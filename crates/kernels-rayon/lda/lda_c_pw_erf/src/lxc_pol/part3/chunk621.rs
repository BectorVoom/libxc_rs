//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 621/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk621(t3619: f64, t589: f64, t25: f64, t3579: f64, t3581: f64, t3583: f64, t3591: f64, t3595: f64, t3600: f64, t3601: f64, t3606: f64, t3611: f64, t3615: f64) -> (f64, f64) {
    let t3620 = t589 * t3619;
    let t3623 = -0.022222222222222223_f64 * t3579 + 0.013333333333333334_f64 * t3581 + 0.0044444444444444444_f64 * t3583 - 0.002962962962962963_f64 * t25 * t3591 - 0.006666666666666667_f64 * t25 * t3595 - t3600 - 0.02666666666666667_f64 * t3601 + 0.013333333333333334_f64 * t25 * t3606 - 0.006666666666666667_f64 * t25 * t3611 - 0.04_f64 * t25 * t3615 + 0.04_f64 * t25 * t3620;
    (t3620, t3623)
}

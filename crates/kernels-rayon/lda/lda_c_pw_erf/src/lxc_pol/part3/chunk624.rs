//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 624/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk624(t3623: f64, t3654: f64, t582: f64, t186: f64, t211: f64, t1518: f64, t550: f64, t548: f64, t594: f64, t580: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3655 = t3623 + t3654;
    let t3656 = t582 * t3655;
    let t3657 = t186 * t3656;
    let t3659 = 2.0_f64 / 15.0_f64 * t211 * t3657;
    let t3660 = t1518 * t550;
    let t3661 = t548 * t3660;
    let t3662 = 8.0_f64 / 45.0_f64 * t3661;
    let t3663 = t1518 * t594;
    let t3664 = t211 * t3663;
    let t3665 = 4.0_f64 / 45.0_f64 * t3664;
    let t3666 = t580 * t580;
    let t3667 = 1.0_f64 / t3666;
    (t3655, t3656, t3657, t3659, t3660, t3661, t3662, t3663, t3664, t3665, t3666, t3667)
}

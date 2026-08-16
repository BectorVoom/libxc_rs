//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 630/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk630(t3709: f64, t525: f64, t1336: f64, t1472: f64, t219: f64, t3604: f64, t2967: f64, t1485: f64, t571: f64, t1341: f64, t1446: f64, t267: f64, t3571: f64, t3573: f64, t3575: f64, t3578: f64, t3659: f64, t3662: f64, t3665: f64, t3673: f64, t3681: f64, t3682: f64, t3684: f64, t3701: f64, t3706: f64, t3708: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3711 = 4.0_f64 / 15.0_f64 * t3709 * t525;
    let t3713 = 8.0_f64 / 15.0_f64 * t1472 * t1336;
    let t3714 = t219 * t3604;
    let t3715 = t3714 * t2967;
    let t3716 = t1485 * t3715;
    let t3718 = 8.0_f64 / 9.0_f64 * t571 * t3716;
    let t3720 = 8.0_f64 / 15.0_f64 * t1446 * t1341;
    let t3721 = -t3571 + t3573 - t3575 - t3578 - t3659 - t3662 + t3665 - t3673 - t3681 + 2.0_f64 / 45.0_f64 * t3682 - 2.0_f64 / 15.0_f64 * t3684 - t3701 * t267 / 15.0_f64 - t3706 + t3708 + t3711 - t3713 - t3718 - t3720;
    (t3711, t3713, t3715, t3716, t3718, t3720, t3721)
}

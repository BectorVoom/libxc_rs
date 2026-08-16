//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1109/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1109(t15542: f64, t15557: f64, t15559: f64, t15568: f64, t15570: f64, t11678: f64, t20670: f64, t20674: f64, t20676: f64, t20678: f64, t20679: f64, t20680: f64, t20681: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20682 = 8.0_f64 / 45.0_f64 * t15542;
    let t20683 = 16.0_f64 / 15.0_f64 * t15557;
    let t20684 = 16.0_f64 / 15.0_f64 * t15559;
    let t20685 = 16.0_f64 / 135.0_f64 * t15568;
    let t20686 = 16.0_f64 / 135.0_f64 * t15570;
    let t20687 = t20670 + t20674 + t20676 + t20678 + t11678 - t20679 + t20680 + t20681 + t20682 - t20683 - t20684 - t20685 - t20686;
    (t20682, t20683, t20684, t20685, t20686, t20687)
}

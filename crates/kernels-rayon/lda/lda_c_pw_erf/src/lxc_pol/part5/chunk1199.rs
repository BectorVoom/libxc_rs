//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1199/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1199(t17692: f64, t17694: f64, t17697: f64, t17699: f64, t17709: f64, t17715: f64, t21717: f64, t21719: f64, t21721: f64, t21725: f64, t21726: f64, t21727: f64, t21728: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21729 = 16.0_f64 / 45.0_f64 * t17692;
    let t21730 = 32.0_f64 / 45.0_f64 * t17694;
    let t21731 = 32.0_f64 / 45.0_f64 * t17697;
    let t21732 = 32.0_f64 / 45.0_f64 * t17699;
    let t21733 = 16.0_f64 / 135.0_f64 * t17709;
    let t21734 = 32.0_f64 / 45.0_f64 * t17715;
    let t21735 = -t21717 - t21719 - t21721 + t21725 - t21726 + t21727 - t21728 - t21729 + t21730 - t21731 + t21732 + t21733 + t21734;
    (t21729, t21730, t21731, t21732, t21733, t21734, t21735)
}

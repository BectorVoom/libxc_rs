//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 764/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk764(t4684: f64, t589: f64, t190: f64, t25: f64, t3469: f64, t3579: f64, t3581: f64, t3583: f64, t3600: f64, t3601: f64, t3627: f64, t3629: f64, t3631: f64, t3639: f64, t3646: f64, t4673: f64, t4691: f64, t4695: f64, t4699: f64, t4981: f64, t4988: f64) -> (f64, f64) {
    let t4991 = t589 * t4684;
    let t4995 = -t3600 - t3639 - 0.014814814814814815_f64 * t3579 + 0.0044444444444444444_f64 * t3581 + 0.0014814814814814814_f64 * t3583 - 0.008888888888888889_f64 * t3601 - 0.03199259259259259_f64 * t3627 + 0.011997222222222222_f64 * t3629 + 0.007998148148148148_f64 * t3631 - 0.023994444444444443_f64 * t3646 + 0.013333333333333334_f64 * t190 * t3469 * t4981 + 0.07198333333333333_f64 * t4699 + 0.07198333333333333_f64 * t4695 - 0.2879333333333333_f64 * t4691 + 0.013333333333333334_f64 * t25 * t4988 - 0.04_f64 * t25 * t4991 + 0.14396666666666666_f64 * t4673;
    (t4991, t4995)
}

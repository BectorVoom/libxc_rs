//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1172/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1172(t13115: f64, t13117: f64, t5160: f64, t10027: f64, t5167: f64, t10438: f64, t13764: f64, t13765: f64, t13766: f64, t13768: f64, t13769: f64, t13770: f64, t13776: f64, t13781: f64, t13784: f64, t13787: f64) -> (f64, f64, f64) {
    let t13790 = 64.0_f64 / 15.0_f64 * t13115 * t5160 * t13117;
    let t13792 = 16.0_f64 / 9.0_f64 * t10027 * t5167;
    let t13793 = -t13764 + t13765 + t13766 - t13768 - t13769 - t10438 - t13770 + t13776 - t13781 - t13784 - t13787 + t13790 + t13792;
    (t13790, t13792, t13793)
}

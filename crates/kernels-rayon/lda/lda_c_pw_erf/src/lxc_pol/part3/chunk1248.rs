//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1248/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1248(t10: f64, t14668: f64, t14807: f64, t14814: f64, t14817: f64, t14819: f64, t14822: f64, t14837: f64, t1568: f64, t1664: f64, t1856: f64, t3251: f64, t411: f64, t426: f64, t5565: f64, t5578: f64, t767: f64) -> f64 {
    let t14839 = 6.0_f64 * t14807 - 18.0_f64 * t426 * t10 * t5578 * t1664 + t14814 + t14817 - 3.0_f64 * t14819 - 3.0_f64 / 2.0_f64 * t14822 + 9.0_f64 / 2.0_f64 * t426 * t10 * t5565 * t411 + 9.0_f64 / 2.0_f64 * t426 * t10 * t1856 * t1568 + 3.0_f64 / 2.0_f64 * t426 * t10 * t767 * t3251 - 17.62848_f64 * t14837 + t14668;
    t14839
}

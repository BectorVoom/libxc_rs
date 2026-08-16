//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1301/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1301(t11159: f64, t11160: f64, t11162: f64, t11164: f64, t11166: f64, t11168: f64, t13465: f64, t13466: f64, t13467: f64, t13469: f64, t13471: f64, t13475: f64, t13477: f64) -> f64 {
    let t15092 = t11159 + 2.0_f64 / 9.0_f64 * t11160 + 4.0_f64 / 3.0_f64 * t11162 - 2.0_f64 / 9.0_f64 * t11164 - 2.0_f64 / 3.0_f64 * t11166 - 0.040518518518518516_f64 * t11168 - t13465 + t13466 - t13467 + t13469 + t13471 - t13475 - t13477;
    t15092
}

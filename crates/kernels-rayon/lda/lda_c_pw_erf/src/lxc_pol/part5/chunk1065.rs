//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1065/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1065(t43: f64, t1781: f64, t19994: f64, t19997: f64, t20007: f64, t348: f64, t4352: f64, t4355: f64, t47: f64, t5982: f64, t5992: f64, t7354: f64, t7360: f64, t8315: f64, t939: f64, t943: f64, zeta_threshold: f64) -> f64 {
    let t44 = t43 <= zeta_threshold;
    let t20011 = piecewise3(t44, 0.0_f64, 40.0_f64 / 81.0_f64 * t8315 * t7354 * t348 - 16.0_f64 / 9.0_f64 * t5982 * t943 - 8.0_f64 / 9.0_f64 * t4352 * t19994 + 8.0_f64 / 3.0_f64 * t4355 * t19997 + 4.0_f64 / 3.0_f64 * t1781 * t5992 + 4.0_f64 / 9.0_f64 * t939 * t7360 * t348 + 4.0_f64 / 3.0_f64 * t47 * t20007);
    t20011
}

//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 598/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk598(t3975: f64, t558: f64, t1410: f64, t640: f64, t653: f64, t254: f64, t474: f64, t252: f64, t3542: f64, t3638: f64, t1519: f64, t511: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3976 = t3975 * t558;
    let t3985 = t640 * t1410;
    let t3988 = 2.0_f64 / 9.0_f64 * t653 * t1410;
    let t3990 = t254 * t474;
    let t3992 = 8.0_f64 / 81.0_f64 * t252 * t3990;
    let t3997 = 0.005877407407407408_f64 * t3542;
    let t4013 = 0.005877407407407408_f64 * t3638;
    let t4029 = t511 * t1519;
    (t3976, t3985, t3988, t3990, t3992, t3997, t4013, t4029)
}

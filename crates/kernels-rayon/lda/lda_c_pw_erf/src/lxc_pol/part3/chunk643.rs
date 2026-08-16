//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 643/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk643(t1314: f64, t3802: f64, t519: f64, t1390: f64, t522: f64, t1392: f64, t505: f64, t1252: f64, t542: f64, t1313: f64, t1329: f64, t3794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3803 = t3802 * t1314;
    let t3804 = t519 * t3803;
    let t3805 = 16.0_f64 / 45.0_f64 * t3804;
    let t3806 = t522 * t1390;
    let t3807 = t505 * t1392;
    let t3808 = t3806 * t3807;
    let t3810 = 8.0_f64 / 15.0_f64 * t519 * t3808;
    let t3811 = t1252 * t542;
    let t3812 = t1313 * t3811;
    let t3814 = 8.0_f64 / 15.0_f64 * t519 * t3812;
    let t3816 = 16.0_f64 / 15.0_f64 * t3794 * t1329;
    (t3803, t3804, t3805, t3806, t3807, t3808, t3810, t3811, t3812, t3814, t3816)
}

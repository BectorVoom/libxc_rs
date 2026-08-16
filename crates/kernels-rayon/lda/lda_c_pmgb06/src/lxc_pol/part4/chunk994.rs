//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 994/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk994(t628: f64, t8165: f64, t4641: f64, t4913: f64, t8697: f64, t8699: f64, t8702: f64, t8704: f64, t8710: f64, t8712: f64, t8714: f64, t675: f64, t682: f64, t696: f64) -> (f64, f64, f64) {
    let t8716 = t628 * t8165;
    let t8719 = -2.8769444444444443_f64 * t8697 + 27.618666666666666_f64 * t8699 - 10.229135802469136_f64 * t8702 + 8.950493827160495_f64 * t8704 + 3.131074074074074_f64 * t4641 + 0.0366775_f64 * t8710 - 0.58684_f64 * t8712 + 0.6520444444444444_f64 * t8714 + 0.5705388888888889_f64 * t8716 + 1.3490888888888888_f64 * t4913;
    let t8723 = 0.5848223622634646_f64 * t696 * t675 * t8719 * t682;
    (t8716, t8719, t8723)
}

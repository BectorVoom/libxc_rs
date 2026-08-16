//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 864/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk864(t8701: f64, t950: f64, t628: f64, t8165: f64, t4641: f64, t4913: f64, t8697: f64, t8699: f64, t8702: f64, t8704: f64, t8710: f64, t8712: f64) -> (f64, f64, f64) {
    let t8714 = t950 * t8701;
    let t8716 = t628 * t8165;
    let t8719 = -2.8769444444444443_f64 * t8697 + 27.618666666666666_f64 * t8699 - 10.229135802469136_f64 * t8702 + 8.950493827160495_f64 * t8704 + 3.131074074074074_f64 * t4641 + 0.0366775_f64 * t8710 - 0.58684_f64 * t8712 + 0.6520444444444444_f64 * t8714 + 0.5705388888888889_f64 * t8716 + 1.3490888888888888_f64 * t4913;
    (t8714, t8716, t8719)
}

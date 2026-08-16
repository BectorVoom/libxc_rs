//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1088/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1088(t26543: f64, t686: f64, t7058: f64, t213: f64, t7398: f64, t2061: f64, t822: f64, t25394: f64, t25402: f64, t7056: f64, t10073: f64, t7064: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26544 = t26543 * t686;
    let t26545 = t7058 * t26544;
    let t26547 = t213 * t7398;
    let t26550 = t822 * t2061;
    let t26551 = t26550 * t25394;
    let t26554 = t25402 * t2061;
    let t26555 = t7056 * t26554;
    let t26557 = 0.24093411633903331839e-3_f64 * t10073 * t26555;
    let t26558 = t7064 * t26544;
    (t26544, t26545, t26547, t26550, t26551, t26554, t26555, t26557, t26558)
}

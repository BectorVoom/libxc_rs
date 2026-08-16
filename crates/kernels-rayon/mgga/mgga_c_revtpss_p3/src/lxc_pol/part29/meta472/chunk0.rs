//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1737/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1737(t25394: f64, t26550: f64, t2061: f64, t25402: f64, t7056: f64, t10073: f64, t26544: f64, t7064: f64, t7384: f64, t887: f64, t689: f64, t7399: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26551 = t26550 * t25394;
    let t26554 = t25402 * t2061;
    let t26555 = t7056 * t26554;
    let t26557 = 0.24093411633903331839e-3_f64 * t10073 * t26555;
    let t26558 = t7064 * t26544;
    let t26560 = t7384 * t887;
    let t26561 = t689 * t26560;
    let t26563 = t786 * t7399;
    (t26551, t26554, t26555, t26557, t26558, t26560, t26561, t26563)
}

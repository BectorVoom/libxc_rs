//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 770/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk770(t1392: f64, t1395: f64, t4045: f64, t4050: f64, t4053: f64, t539: f64, t541: f64, t5650: f64, t9872: f64, t9881: f64, t9884: f64, t9887: f64) -> f64 {
    let t9890 = -36.0_f64 * t1392 * t4050 + 9.0_f64 * t1392 * t4053 + 9.0_f64 * t1395 * t4045 + 60.0_f64 * t539 * t9881 + 3.0_f64 * t539 * t9887 - t541 * t9872 - 36.0_f64 * t5650 * t9884;
    t9890
}

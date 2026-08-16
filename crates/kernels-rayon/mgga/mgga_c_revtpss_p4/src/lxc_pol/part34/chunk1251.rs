//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1251/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1251(t27450: f64, t4820: f64, t20054: f64, t7132: f64, t20050: f64, t19785: f64, t25517: f64, t6317: f64, t7131: f64, t19826: f64, t25509: f64, t20029: f64, t25505: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t106926 = t27450 * t4820;
    let t106934 = t7132 * t20054;
    let t106938 = t7132 * t20050;
    let t106960 = t25517 * t19785;
    let t106971 = t6317 * t7131;
    let t107015 = t25509 * t19826;
    let t107027 = t25505 * t20029;
    (t106926, t106934, t106938, t106960, t106971, t107015, t107027)
}

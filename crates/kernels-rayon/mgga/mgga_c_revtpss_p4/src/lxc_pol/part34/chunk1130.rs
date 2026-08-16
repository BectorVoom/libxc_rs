//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1130/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1130(t3252: f64, t65: f64, t3204: f64, t7131: f64, t4817: f64, t7132: f64, t7810: f64, t994: f64, t1976: f64, t4746: f64, t1035: f64, t1982: f64, t27418: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27531 = t65 * t3252;
    let t27536 = t3204 * t7131;
    let t27539 = t7132 * t4817;
    let t27550 = t994 * t7810;
    let t27568 = t4746 * t1976;
    let t27604 = t1035 * t7810;
    let t27609 = t1982 * t27418;
    (t27531, t27536, t27539, t27550, t27568, t27604, t27609)
}

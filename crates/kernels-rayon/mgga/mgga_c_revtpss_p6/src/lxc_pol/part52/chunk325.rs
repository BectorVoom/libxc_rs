//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 325/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk325(t1468: f64, t100: f64, t55: f64, t108: f64, t105: f64, t109: f64, t97: f64, tau1: f64) -> (f64, f64, f64, f64, f64) {
    let t1504 = t1468 / 2.0_f64;
    let t1505 = t100 * t1504;
    let t1507 = tau1 * t55;
    let t1509 = -t1504;
    let t1510 = t108 * t1509;
    let t1513 = 5.0_f64 / 3.0_f64 * t105 * t1510 - 5.0_f64 / 3.0_f64 * t1507 * t109 + 5.0_f64 / 3.0_f64 * t97 * t1505;
    (t1504, t1505, t1507, t1509, t1513)
}

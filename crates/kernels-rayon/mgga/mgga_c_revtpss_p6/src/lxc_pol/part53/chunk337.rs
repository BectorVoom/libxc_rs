//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 337/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk337(t1470: f64, t70: f64, t1469: f64, t48: f64, t51: f64, t53: f64, t60: f64, t44: f64, t56: f64, t61: f64, t626: f64, t38: f64, rho1: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1471 = t1470 * t70;
    let t1474 = t48 * t1469;
    let t1477 = t51 * rho1;
    let t1479 = 1.0_f64 / t53 / t1477;
    let t1480 = sigma2 * t1479;
    let t1483 = t60 * t1469;
    let t1486 = 5.0_f64 / 6.0_f64 * t44 * t1474 - 8.0_f64 / 3.0_f64 * t1480 * t61 - 5.0_f64 / 6.0_f64 * t56 * t1483 + t626;
    let t1487 = t38 * t1486;
    (t1471, t1474, t1479, t1480, t1486, t1487)
}

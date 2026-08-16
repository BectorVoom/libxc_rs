//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1666/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1666(t88202: f64, t923: f64, t141: f64, t2908: f64, t88124: f64, t88087: f64, t930: f64, t52128: f64, t63453: f64, t63459: f64, t63464: f64, t63533: f64, t63538: f64, t63545: f64, t77559: f64, t77561: f64, t77806: f64, t77858: f64) -> (f64, f64, f64, f64) {
    let t88252 = t923 * t88202;
    let t88257 = t141 * t2908 * t88124;
    let t88260 = t141 * t930 * t88087;
    let t88262 = 0.22076e0_f64 * t77806 + 0.98115555555555555556e0_f64 * t52128 - 0.53675555555555555556e0_f64 * t63453 + 0.16102666666666666667e1_f64 * t63459 - 0.18396666666666666667e0_f64 * t63533 + 0.11038e1_f64 * t63538 - 0.5519e0_f64 * t63545 + 0.80513333333333333333e0_f64 * t77559 - 0.24154e1_f64 * t77561 + 0.16504875e0_f64 * t88252 - 0.80513333333333333336e0_f64 * t63464 + 0.22076e0_f64 * t77858 + 0.99342e0_f64 * t88257 - 0.298026e1_f64 * t88260;
    (t88252, t88257, t88260, t88262)
}

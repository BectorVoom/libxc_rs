//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 478/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk478(t1470: f64, t70: f64, t1469: f64, t48: f64, t51: f64, t53: f64, rho1: f64, sigma2: f64) -> (f64, f64, f64) {
    let t1471 = t1470 * t70;
    let t1474 = t48 * t1469;
    let t1477 = t51 * rho1;
    let t1479 = 1.0_f64 / t53 / t1477;
    let t1480 = sigma2 * t1479;
    (t1471, t1474, t1480)
}

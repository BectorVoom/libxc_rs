//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 126/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk126(t473: f64, t51: f64, t52: f64, rho1: f64, sigma2: f64) -> (f64, f64, f64, f64, f64) {
    let t474 = sigma2 * sigma2;
    let t475 = t473 * t474;
    let t476 = t51 * t51;
    let t477 = t476 * rho1;
    let t479 = 1.0_f64 / t52 / t477;
    let t480 = t475 * t479;
    (t474, t475, t476, t479, t480)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2954/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2954(t300: f64, t77637: f64, t77873: f64, t78155: f64, t78196: f64, t78240: f64, t78279: f64, t78316: f64, t78398: f64, t77492: f64, t77494: f64, t77496: f64, t77498: f64, t77600: f64, t77604: f64, t77612: f64, t77622: f64, t77624: f64, t77628: f64) -> (f64, f64) {
    let t78402 = t300 * (t77637 + t77873 + t78155 + t78196 + t78240 + t78279 + t78316 + t78398);
    let t78403 = -t77492 - t77494 - t77496 - t77498 + t77600 - t77604 + t78402 - t77612 + t77622 + t77624 + t77628;
    (t78402, t78403)
}

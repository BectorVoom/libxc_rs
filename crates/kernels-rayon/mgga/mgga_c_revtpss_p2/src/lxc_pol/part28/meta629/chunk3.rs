//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2268/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2268(t101432: f64, t101555: f64, t97635: f64, t98422: f64, t98468: f64, t98512: f64, t98563: f64, t98612: f64, t1913: f64, t7337: f64, t1916: f64, t26120: f64) -> (f64, f64, f64) {
    let t101558 = t97635 + t98422 + t98468 + t98512 + t98563 + t98612 + t101432 + t101555;
    let t101563 = 2.0_f64 * t1913 * t7337;
    let t101568 = 6.0_f64 * t1916 * t26120;
    (t101558, t101563, t101568)
}

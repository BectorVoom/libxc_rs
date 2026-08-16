//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 872/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk872(t22671: f64, t36: f64, t70: f64, t1486: f64, t5826: f64, t1470: f64, t5854: f64, t1469: f64, t5819: f64) -> (f64, f64, f64, f64, f64) {
    let t22672 = t36 * t22671;
    let t22673 = t22672 * t70;
    let t22676 = t5826 * t1486;
    let t22681 = t1470 * t5854;
    let t22688 = t5819 * t1469;
    (t22672, t22673, t22676, t22681, t22688)
}

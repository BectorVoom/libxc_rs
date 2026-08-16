//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1939/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1939(t25220: f64, t25232: f64, t25243: f64, t28330: f64, t28333: f64, t28335: f64, t28336: f64, t29616: f64, t29618: f64, t29620: f64, t29635: f64) -> f64 {
    let t29636 = t25220 - t25232 + t25243 + t28330 + 0.85748036236139473944e-3_f64 * t29616 + 0.34299214494455789578e-2_f64 * t29618 - 0.42874018118069736972e-3_f64 * t29620 - t28335 + t28336 + t28333 + t29635;
    t29636
}

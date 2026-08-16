//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 296/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk296(t797: f64, t799: f64, t802: f64, t812: f64, t819: f64, t825: f64, t839: f64, t848: f64, t851: f64, t857: f64) -> f64 {
    let t860 = -t797 - t799 * t802 / 48.0_f64 - t812 + t819 - 0.21437009059034868486e-3_f64 * t825 * t839 - t848 - 0.85748036236139473944e-3_f64 * t851 * t857;
    t860
}

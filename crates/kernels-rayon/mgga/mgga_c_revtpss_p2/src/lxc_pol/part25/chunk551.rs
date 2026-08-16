//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 551/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk551(t357: f64, t905: f64, t606: f64, t3093: f64, t3092: f64) -> (f64, f64, f64, f64) {
    let t3094 = t357 * t905;
    let t3095 = t3094 * t606;
    let t3096 = t3093 * t3095;
    let t3097 = t3092 * t3096;
    (t3094, t3095, t3096, t3097)
}

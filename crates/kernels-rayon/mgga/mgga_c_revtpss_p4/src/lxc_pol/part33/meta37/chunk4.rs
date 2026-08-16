//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 252/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk252(t123: f64, t173: f64, t186: f64, t676: f64, t679: f64, t704: f64, t724: f64, t731: f64, t739: f64, t746: f64) -> f64 {
    let t749 = 0.53237641966666666666e-3_f64 * t123 * t676 * t173 + 1.0_f64 * t724 * t731 - t679 - t704 + 0.18311447306006545054e-3_f64 * t123 * t676 * t186 + 0.5848223622634646207e0_f64 * t739 * t746;
    t749
}

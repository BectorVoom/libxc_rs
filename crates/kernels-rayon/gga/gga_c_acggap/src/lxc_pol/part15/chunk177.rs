//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 177/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk177(t127: f64, t332: f64, t335: f64, t367: f64, t380: f64, t392: f64, t397: f64, t417: f64, t418: f64, t500: f64, t509: f64, t515: f64, t527: f64, t532: f64, t537: f64, t542: f64) -> f64 {
    let t545 = -t332 - t335 * t500 / 48.0_f64 + t127 * t509 / 96.0_f64 - t367 * t515 / 96.0_f64 + t380 - t392 - 0.21437009059034868486e-3_f64 * t397 * t527 - t417 - 0.85748036236139473944e-3_f64 * t418 * t532 + 0.42874018118069736972e-3_f64 * t418 * t537 - 0.42874018118069736972e-3_f64 * t418 * t542;
    t545
}

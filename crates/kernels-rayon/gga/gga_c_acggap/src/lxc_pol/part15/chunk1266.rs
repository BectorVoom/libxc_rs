//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1266/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1266(t31855: f64, t32955: f64, t32967: f64, t36340: f64, t36349: f64, t36370: f64, t37971: f64, t37972: f64, t37973: f64, t37979: f64, t37980: f64, t37983: f64, t37985: f64, t40533: f64, t40537: f64, t40542: f64, t40546: f64, t40549: f64) -> f64 {
    let t42160 = 0.15724046144802076034e-2_f64 * t40533 + 0.15724046144802076034e-2_f64 * t40537 - t32955 + t36340 - 0.6289618457920830414e-2_f64 * t40542 - 0.90702367218671976884e-1_f64 * t36349 - t37971 + t37972 + t37973 + 0.34299214494455789578e-2_f64 * t31855 + t37979 + t37980 + t40546 / 48.0_f64 - 0.68598428988911579156e-2_f64 * t36370 - t37983 + t32967 - t37985 + 0.85748036236139473944e-3_f64 * t40549;
    t42160
}

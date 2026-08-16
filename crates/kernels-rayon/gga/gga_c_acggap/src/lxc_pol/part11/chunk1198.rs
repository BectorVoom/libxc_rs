//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1198/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1198(t36388: f64, t1967: f64, t8566: f64, t1998: f64, t4557: f64, t31859: f64, t31864: f64, t31868: f64, t31870: f64, t31872: f64, t36365: f64, t36368: f64, t36370: f64, t36373: f64, t36374: f64, t36378: f64, t36381: f64, t36383: f64, t36385: f64, t36386: f64) -> f64 {
    let t36389 = 0.34299214494455789578e-2_f64 * t36388;
    let t36390 = t1967 * t8566;
    let t36391 = 0.37737710747524982482e-2_f64 * t36390;
    let t36392 = t1998 * t4557;
    let t36394 = 0.42874018118069736972e-3_f64 * t31859 + t36365 + t36368 + 0.85748036236139473944e-3_f64 * t31864 - 0.17149607247227894789e-2_f64 * t36370 - t36373 - 0.17149607247227894789e-2_f64 * t36374 + t31868 - t36378 + t31870 / 16.0_f64 + t36381 + t36383 - 7.0_f64 / 288.0_f64 * t31872 - t36385 + 0.27953859812981468505e-2_f64 * t36386 + t36389 + t36391 + 0.17149607247227894789e-2_f64 * t36392;
    t36394
}

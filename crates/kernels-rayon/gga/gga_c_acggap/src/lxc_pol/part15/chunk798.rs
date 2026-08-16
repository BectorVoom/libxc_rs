//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 798/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk798(t8556: f64, t8574: f64, t8580: f64, t8582: f64, t7317: f64, t7319: f64, t8558: f64, t8562: f64, t8567: f64, t8572: f64, t8576: f64, t8578: f64, t8584: f64, t8586: f64, t8590: f64) -> f64 {
    let t9206 = 0.10482697429868050689e-2_f64 * t8556;
    let t9211 = 0.85748036236139473944e-3_f64 * t8574;
    let t9214 = 0.18868855373762491241e-2_f64 * t8580;
    let t9215 = 0.21437009059034868486e-3_f64 * t8582;
    let t9219 = t9206 - 0.62896184579208304138e-3_f64 * t8558 - 0.62896184579208304138e-3_f64 * t8562 - 0.62896184579208304138e-3_f64 * t8567 - 0.41930789719472202759e-3_f64 * t8572 - t9211 - 0.85748036236139473944e-3_f64 * t8576 + 0.94344276868812456207e-3_f64 * t8578 - t9214 - t9215 - t8584 / 48.0_f64 - t8586 / 48.0_f64 - t8590 / 64.0_f64 + t7317 + t7319;
    t9219
}

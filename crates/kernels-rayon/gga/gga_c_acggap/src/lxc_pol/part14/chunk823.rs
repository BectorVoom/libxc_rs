//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 823/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk823(t598: f64, t9577: f64, t7328: f64, t7373: f64, t7376: f64, t9214: f64, t9215: f64, t9222: f64, t9532: f64, t9534: f64, t9539: f64, t9544: f64, t9546: f64, t9550: f64, t9555: f64, t9557: f64, t9561: f64, t9566: f64, t9568: f64, t9570: f64, t9574: f64) -> f64 {
    let t9578 = t598 * t9577;
    let t9580 = -t9214 - t9215 + 0.32155513588552302729e-2_f64 * t9532 - 0.42874018118069736972e-3_f64 * t9534 + 0.7862023072401038017e-3_f64 * t9539 - 0.31448092289604152068e-3_f64 * t9544 - 0.42874018118069736972e-3_f64 * t9546 - 0.10718504529517434243e-2_f64 * t9550 - 0.18868855373762491241e-2_f64 * t9555 + 0.85748036236139473944e-3_f64 * t9557 - 0.42874018118069736972e-3_f64 * t9561 - 0.21437009059034868486e-3_f64 * t9566 - 0.68598428988911579156e-2_f64 * t9568 + 0.68598428988911579156e-2_f64 * t9570 - t7328 + t9222 + 0.64311027177104605458e-3_f64 * t9574 - 0.47172138434406228102e-2_f64 * t9578 + t7373 - t7376;
    t9580
}

//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1228/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1228(t30725: f64, t32561: f64, t34751: f64, t37230: f64, t37234: f64, t39438: f64, t39442: f64, t39447: f64, t39451: f64, t39454: f64, t39458: f64, t39462: f64, t39465: f64, t39468: f64, t39471: f64, t39474: f64, t39477: f64) -> f64 {
    let t41664 = -t37230 + 0.10482697429868050689e-2_f64 * t39438 + 0.68598428988911579156e-2_f64 * t34751 + 0.62896184579208304138e-3_f64 * t39442 + t37234 + 0.31448092289604152069e-2_f64 * t30725 + t32561 - 0.31448092289604152068e-2_f64 * t39447 + 0.57165357490759649296e-3_f64 * t39451 + 0.85748036236139473944e-3_f64 * t39454 + 0.12579236915841660828e-2_f64 * t39458 + 0.12579236915841660828e-2_f64 * t39462 - t39465 / 8.0_f64 + t39468 / 4.0_f64 + t39471 / 12.0_f64 + t39474 / 8.0_f64 + t39477 / 24.0_f64;
    t41664
}

//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 199/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk199(t211: f64, t88: f64, t238: f64, t233: f64, t352: f64, t354: f64, t358: f64, t360: f64, t241: f64, t374: f64, t46: f64, t379: f64, t381: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t689 = t211 * t88;
    let t704 = t238 * t238;
    let t705 = 1.0_f64 / t704;
    let t706 = t233 * t705;
    let t711 = -0.1176575e1_f64 * t352 - 0.516475e0_f64 * t354 - 0.2103875e0_f64 * t358 - 0.104195e0_f64 * t360;
    let t712 = 1.0_f64 / t241;
    let t713 = t711 * t712;
    let t719 = t46 * t374;
    let t720 = t379 * t381;
    (t689, t704, t705, t706, t711, t712, t713, t719, t720)
}

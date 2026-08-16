//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 111/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk111(t256: f64, t265: f64, t234: f64, t241: f64, t243: f64, t252: f64, t136: f64, t96: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t235 = 2.0_f64 <= zeta_threshold;
    let t238 = 0.0_f64 <= zeta_threshold;
    let t266 = t256 * t265;
    let t269 = t241 * (-0.3109e-1_f64 * t243 * t252 + t234 - 0.19751789702565206229e-1_f64 * t266);
    let t271 = 0.19751789702565206229e-1_f64 * t241 * t266;
    let t272 = piecewise3(t235, t96, t136);
    let t273 = piecewise3(t238, t96, 0.0_f64);
    let t275 = t272 / 2.0_f64 + t273 / 2.0_f64;
    (t269, t271, t275)
}

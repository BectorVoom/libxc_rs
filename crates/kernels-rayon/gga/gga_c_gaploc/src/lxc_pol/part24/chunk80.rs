//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 80/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk80(t204: f64, t61: f64, t190: f64, t193: f64, t199: f64, t64: f64, rho0: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t205 = t61 * t204;
    let t208 = 1.0_f64 + 0.35750489951850426669e0_f64 * t190 * t193 - 0.11502877786176224903e1_f64 * t199 * t205;
    let t209 = 1.0_f64 / t208;
    let t211 = rho0 - rho1;
    let t212 = t211 * t64;
    let t213 = 1.0_f64 + t212;
    let t214 = t213 <= zeta_threshold;
    let t215 = pow_1_3(t213);
    (t205, t208, t209, t211, t212, t213, t215)
}

//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 61/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk61<F: Float>(t204: F, t61: F, t190: F, t193: F, t199: F, t64: F, t36: F, t43: F, t40: F, rho0: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t205 = t61 * t204;
    let t208 = F::cast_from(1.0_f64) + F::cast_from(0.35750489951850426669e0_f64) * t190 * t193 - F::cast_from(0.11502877786176224903e1_f64) * t199 * t205;
    let t209 = F::cast_from(1.0_f64) / t208;
    let t211 = rho0 - rho1;
    let t212 = t211 * t64;
    let t213 = F::cast_from(1.0_f64) + t212;
    let t214 = t213 <= zeta_threshold;
    let t215 = pow_1_3::<F>(t213);
    let t217 = piecewise3::<F>(t214, t36, t215 * t213);
    let t218 = F::cast_from(1.0_f64) - t212;
    let t219 = t218 <= zeta_threshold;
    let t220 = pow_1_3::<F>(t218);
    let t222 = piecewise3::<F>(t219, t36, t220 * t218);
    let t224 = (t217 + t222 - F::cast_from(2.0_f64)) * t43;
    let t225 = F::cast_from(2.0_f64) <= zeta_threshold;
    let t227 = piecewise3::<F>(t225, t36, F::cast_from(2.0_f64) * t40);
    let t228 = F::cast_from(0.0_f64) <= zeta_threshold;
    let t229 = piecewise3::<F>(t228, t36, F::cast_from(0.0_f64));
    let t231 = (t227 + t229 - F::cast_from(2.0_f64)) * t43;
    (t205, t208, t209, t211, t215, t220, t224, t231, t213, t218)
}

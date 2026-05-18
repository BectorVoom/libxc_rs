//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 80/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk80<F: Float>(t204: F, t61: F, t190: F, t193: F, t199: F, t64: F, rho0: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t205 = t61 * t204;
    let t208 = F::new(1.0) + F::new(0.35750489951850426669e0) * t190 * t193 - F::new(0.11502877786176224903e1) * t199 * t205;
    let t209 = F::new(1.0) / t208;
    let t211 = rho0 - rho1;
    let t212 = t211 * t64;
    let t213 = F::new(1.0) + t212;
    let t214 = t213 <= zeta_threshold;
    let t215 = pow_1_3::<f64>(t213);
    (t205, t208, t209, t211, t212, t213, t215)
}

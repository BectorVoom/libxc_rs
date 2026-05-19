//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 111/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk111<F: Float>(t256: F, t265: F, t234: F, t241: F, t243: F, t252: F, t136: F, t96: F, zeta_threshold: F) -> (F, F, F) {
    let t235 = F::new(2.0) <= zeta_threshold;
    let t238 = F::new(0.0) <= zeta_threshold;
    let t266 = t256 * t265;
    let t269 = t241 * (-F::new(0.3109e-1) * t243 * t252 + t234 - F::cast_from(0.19751789702565206229e-1_f64) * t266);
    let t271 = F::cast_from(0.19751789702565206229e-1_f64) * t241 * t266;
    let t272 = piecewise3::<F>(t235, t96, t136);
    let t273 = piecewise3::<F>(t238, t96, F::new(0.0));
    let t275 = t272 / F::new(2.0) + t273 / F::new(2.0);
    (t269, t271, t275)
}

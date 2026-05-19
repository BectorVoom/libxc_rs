//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 97/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk97<F: Float>(t252: F, t261: F, t230: F, t237: F, t239: F, t248: F, t136: F, t173: F, zeta_threshold: F) -> (F, F, F, F) {
    let t231 = F::new(2.0) <= zeta_threshold;
    let t234 = F::new(0.0) <= zeta_threshold;
    let t262 = t252 * t261;
    let t265 = t237 * (-F::new(0.310907e-1) * t239 * t248 + t230 - F::cast_from(0.19751673498613801407e-1_f64) * t262);
    let t267 = F::cast_from(0.19751673498613801407e-1_f64) * t237 * t262;
    let t268 = piecewise3::<F>(t231, t136, t173);
    let t269 = piecewise3::<F>(t234, t136, F::new(0.0));
    let t271 = t268 / F::new(2.0) + t269 / F::new(2.0);
    let t272 = t271 * t271;
    (t265, t267, t271, t272)
}

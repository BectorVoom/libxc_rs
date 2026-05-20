//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 136/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk136<F: Float>(t315: F, t324: F, t293: F, t300: F, t302: F, t311: F, t199: F, t240: F, zeta_threshold: F) -> (F, F, F, F) {
    let t294 = F::new(2.0) <= zeta_threshold;
    let t297 = F::new(0.0) <= zeta_threshold;
    let t325 = t315 * t324;
    let t328 = t300 * (-F::new(0.310907e-1) * t302 * t311 + t293 - F::cast_from(0.19751673498613801407e-1_f64) * t325);
    let t330 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t325;
    let t331 = piecewise3::<F>(t294, t199, t240);
    let t332 = piecewise3::<F>(t297, t199, F::new(0.0));
    let t334 = t331 / F::new(2.0) + t332 / F::new(2.0);
    let t335 = t334 * t334;
    (t328, t330, t334, t335)
}

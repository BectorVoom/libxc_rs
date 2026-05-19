//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1105/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1105<F: Float>(t481: F, t9577: F, t39263: F, t4176: F, t792: F, t37327: F, t37377: F, t37380: F, t37387: F, t39188: F, t39195: F, t39201: F, t39205: F, t39208: F, t39212: F, t39247: F, t39252: F, t39256: F, t39261: F) -> (F, F, F) {
    let t39264 = t9577 * t481;
    let t39267 = F::new(3.0) * t39263 * t4176 * t39264;
    let t39268 = t9577 * t792;
    let t39271 = F::new(15.0) / F::new(8.0) * t37327 * t4176 * t39268;
    let t39272 = -F::cast_from(0.36021158228745895953e-3_f64) * t39247 + t39252 + t39256 - t39188 - t39195 + t39201 + t39205 + F::cast_from(0.96056421943322389208e-3_f64) * t37377 + t39208 - F::cast_from(0.40650199722100037752e-3_f64) * t37380 - t39212 - t37387 - t39261 + t39267 - t39271;
    (t39267, t39271, t39272)
}

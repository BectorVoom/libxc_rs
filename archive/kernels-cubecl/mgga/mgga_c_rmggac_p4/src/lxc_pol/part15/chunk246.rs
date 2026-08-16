//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 246/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk246<F: Float>(t1228: F, t492: F, t140: F, t453: F, t73: F, t75: F, t80: F, t295: F, t299: F, t1007: F, t78: F, t76: F) -> (F, F, F, F, F, F, F) {
    let t1229 = t1228 * t492;
    let t1231 = t453 * t140;
    let t1279 = t75 * t73;
    let t1281 = F::cast_from(132.0_f64) * t1279 * t80;
    let t1283 = F::cast_from(288.0_f64) * t295 * t299;
    let t1284 = t78 * t1007;
    let t1285 = F::cast_from(1.0_f64) / t1284;
    let t1287 = F::cast_from(156.0_f64) * t76 * t1285;
    (t1229, t1231, t1279, t1281, t1283, t1285, t1287)
}

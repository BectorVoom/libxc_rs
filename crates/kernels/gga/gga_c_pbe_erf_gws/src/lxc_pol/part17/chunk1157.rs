//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1157/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1157<F: Float>(t2146: F, t54359: F, t14007: F, t9545: F, t51431: F, t54338: F, t54342: F, t54345: F, t54346: F, t54348: F, t54350: F, t54352: F, t54355: F, t54356: F, t9478: F, t14015: F, t9460: F) -> (F, F, F) {
    let t54360 = t2146 * t54359;
    let t54362 = t14007 * t9545;
    let t54364 = -5.0 / 96.0 * t54338 + t54342 / 48.0 - t54345 - 5.0 / 64.0 * t54346 - t54348 / 48.0 - t54350 / 96.0 - 119.0 / 1728.0 * t54352 + t54355 - 35.0 / 216.0 * t54356 + 7.0 / 144.0 * t51431 + t54360 / 8.0 + t54362 / 384.0;
    let t54366 = t14007 * t9478;
    let t54368 = t14015 * t9460;
    (t54364, t54366, t54368)
}

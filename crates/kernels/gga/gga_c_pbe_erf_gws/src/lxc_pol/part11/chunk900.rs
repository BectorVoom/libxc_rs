//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 900/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk900<F: Float>(t1044: F, t3354: F, t12693: F, t401: F, t12696: F, t12477: F, t395: F, t12469: F, t12679: F, t12686: F, t12461: F, t12676: F, t12481: F, t12683: F, t12465: F, t12473: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t40079 = t3354 * t1044;
    let t40105 = t401 * t12693;
    let t40107 = t401 * t12696;
    let t40163 = t395 * t12477;
    let t40213 = t395 * t12469;
    let t40245 = t401 * t12679;
    let t40247 = t401 * t12686;
    let t40251 = t395 * t12461;
    let t40253 = t401 * t12676;
    let t40255 = t395 * t12481;
    let t40260 = t401 * t12683;
    let t40262 = t395 * t12465;
    let t40264 = t395 * t12473;
    (t40079, t40105, t40107, t40163, t40213, t40245, t40247, t40251, t40253, t40255, t40260, t40262, t40264)
}

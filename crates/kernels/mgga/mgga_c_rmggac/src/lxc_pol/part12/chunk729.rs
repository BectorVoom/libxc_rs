//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 729/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk729<F: Float>(t8583: F, t8585: F, t8588: F, t38260: F, t7520: F, t7523: F, t7526: F, t7529: F, t7532: F, t7535: F, t8574: F, t8590: F, t8593: F, t8595: F, t8598: F, t8604: F) -> (F, F, F, F, F, F) {
    let t38261 = 0.85129199786595678796e-5 * t8583;
    let t38262 = 0.25538759935978703638e-4 * t8585;
    let t38263 = 0.25538759935978703638e-4 * t8588;
    let t38264 = 0.25538759935978703638e-4 * t8574 + t7520 + t7523 - t7526 + t7529 + t7532 - t38260 - t7535 - t38261 + t38262 + t38263;
    let t38266 = 0.25538759935978703638e-4 * t8590;
    let t38267 = 0.25538759935978703638e-4 * t8593;
    let t38268 = 0.85129199786595678796e-5 * t8595;
    let t38269 = 0.85129199786595678796e-5 * t8598;
    let t38271 = 0.85129199786595678796e-5 * t8604;
    (t38264, t38266, t38267, t38268, t38269, t38271)
}

//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 806/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk806<F: Float>(t8565: F, t8569: F, t8572: F, t8578: F, t8583: F, t8585: F, t8588: F, t7520: F, t7523: F, t7526: F, t7529: F, t7532: F, t7535: F, t8574: F) -> (F, F, F, F) {
    let t38255 = F::new(0.68186654135613354322e-2) * t8565;
    let t38256 = F::new(0.68186654135613354322e-2) * t8569;
    let t38257 = F::new(0.85129199786595678796e-5) * t8572;
    let t38260 = F::new(0.85129199786595678796e-5) * t8578;
    let t38261 = F::new(0.85129199786595678796e-5) * t8583;
    let t38262 = F::new(0.25538759935978703638e-4) * t8585;
    let t38263 = F::new(0.25538759935978703638e-4) * t8588;
    let t38264 = F::new(0.25538759935978703638e-4) * t8574 + t7520 + t7523 - t7526 + t7529 + t7532 - t38260 - t7535 - t38261 + t38262 + t38263;
    (t38255, t38256, t38257, t38264)
}

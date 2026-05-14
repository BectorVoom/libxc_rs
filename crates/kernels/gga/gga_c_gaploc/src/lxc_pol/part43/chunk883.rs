//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 883/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk883<F: Float>(t40561: F, t42429: F, t42432: F, t42438: F, t42442: F, t42444: F, t42455: F, t42456: F, t42457: F, t42459: F, t42460: F, t42461: F, t48205: F, t48208: F, t48211: F, t48217: F, t48221: F, t48225: F, t48231: F, t48233: F) -> (F,) {
    let t50925 = t42429 - t42432 - 0.18404604457881959845e2 * t48205 - 0.29792074959875355558e-1 * t48208 + 0.13803453343411469884e2 * t48211 + t42438 + t42442 - t42444 - 0.12269736305254639897e2 * t48217 - 0.92023022289409799224e1 * t48221 - 0.92023022289409799224e1 * t48225 - t48231 + t48233 + t42455 - t42456 + t42457 - 0.59584149919750711115e-1 * t40561 + t42459 - t42460 + t42461;
    (t50925,)
}

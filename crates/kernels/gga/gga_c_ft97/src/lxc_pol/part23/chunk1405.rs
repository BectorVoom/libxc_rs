//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1405/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1405<F: Float>(t128162: F, t128173: F, t128186: F, t128192: F, t128203: F, t128211: F, t128223: F, t128235: F, t128249: F, t128258: F, t128269: F, t128279: F, t128293: F, t128305: F, t128319: F, t128330: F) -> (F,) {
    let t128334 = t128162 + t128173 + t128186 + t128192 + t128203 + t128211 + t128223 + t128235 + t128249 + t128258 + t128269 + t128279 + t128293 + t128305 + t128319 + t128330;
    (t128334,)
}

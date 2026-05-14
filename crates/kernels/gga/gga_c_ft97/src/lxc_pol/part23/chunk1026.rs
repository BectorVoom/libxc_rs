//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1026/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1026<F: Float>(t193: F, t24757: F, t28338: F, t28353: F, t31213: F, t31217: F, t31222: F, t31226: F, t31231: F, t31236: F, t31241: F, t31246: F, t31249: F, t31252: F, t31255: F, t31258: F, t31262: F, t31268: F, t446: F, t89: F) -> (F,) {
    let t31271 = 2.0 / 3.0 * t446 * t31213 + 4.0 / 3.0 * t446 * t31217 - 2.0 / 3.0 * t446 * t31222 + 4.0 / 3.0 * t446 * t31226 - 4.0 / 9.0 * t28338 - 2.0 * t446 * t31231 + t446 * t31236 / 3.0 + 2.0 / 3.0 * t446 * t31241 - 2.0 / 3.0 * t446 * t31246 + 4.0 / 3.0 * t446 * t31249 + 4.0 / 3.0 * t446 * t31252 + 2.0 / 3.0 * t446 * t31255 - t24757 - t446 * t31258 / 3.0 + t89 * t193 * t31262 / 3.0 - 2.0 / 27.0 * t28353 + 2.0 / 3.0 * t446 * t31268;
    (t31271,)
}

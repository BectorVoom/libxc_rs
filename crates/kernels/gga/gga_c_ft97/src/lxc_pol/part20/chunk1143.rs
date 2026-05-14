//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1143/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1143<F: Float>(t109437: F, t109431: F, t109434: F, t97207: F, t97209: F, t97214: F, t97220: F, t97232: F, t97235: F, t97238: F, t97244: F, t97412: F, t109442: F, t109469: F, t109446: F, t109451: F, t109455: F, t109459: F, t109463: F, t109467: F, t109473: F, t109476: F, t109479: F, t109483: F) -> (F, F) {
    let t110235 = 2.0 / 9.0 * t109437;
    let t110237 = -t97207 / 27.0 + 2.0 / 27.0 * t97209 + 2.0 / 27.0 * t97214 - t109431 / 3.0 + t97220 / 3.0 + 2.0 / 27.0 * t97232 - t97235 / 18.0 - t97238 / 36.0 - 4.0 / 27.0 * t109434 + t110235 - 8.0 / 27.0 * t97244 + t97412;
    let t110238 = t109442 / 9.0;
    let t110245 = 4.0 / 3.0 * t109469;
    let t110250 = t110238 + t109446 / 9.0 + 2.0 / 27.0 * t109451 + t109455 / 9.0 - 4.0 * t109459 - 2.0 * t109463 - 4.0 * t109467 + t110245 - 4.0 / 9.0 * t109473 - 2.0 / 27.0 * t109476 + 4.0 / 9.0 * t109479 - 4.0 / 27.0 * t109483;
    (t110237, t110250)
}

//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1232/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1232<F: Float>(t112816: F, t2665: F, t3281: F, t3886: F, t99363: F, t10409: F, t446: F, t10248: F, t113041: F, t15407: F, t6334: F, t99511: F, t28822: F, t6308: F, t681: F, t113289: F, t113293: F, t113296: F, t113298: F, t113301: F, t113304: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t113307 = t3281 * t2665 * t112816;
    let t113309 = t99363 * t3886;
    let t113311 = t446 * t10409 * t113309;
    let t113314 = t446 * t10248 * t113041;
    let t113316 = t6334 * t15407;
    let t113318 = t3281 * t10248 * t113316;
    let t113320 = t99511 * t3886;
    let t113322 = t446 * t2665 * t113320;
    let t113325 = t6308 * t681 * t28822;
    let t113326 = t113325 / 6.0;
    let t113327 = -2.0 / 3.0 * t113289 + 24.0 * t113293 - t113296 + 11.0 / 9.0 * t113298 - 2.0 / 3.0 * t113301 - 4.0 / 3.0 * t113304 + 8.0 / 3.0 * t113307 + 4.0 / 9.0 * t113311 - 2.0 / 3.0 * t113314 + 8.0 / 3.0 * t113318 - 4.0 / 3.0 * t113322 - t113326;
    (t113307, t113309, t113311, t113314, t113316, t113318, t113320, t113322, t113325, t113327)
}

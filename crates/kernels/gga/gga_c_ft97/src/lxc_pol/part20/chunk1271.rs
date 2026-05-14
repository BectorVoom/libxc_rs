//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1271/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1271<F: Float>(t113329: F, t113332: F, t113339: F, t113346: F, t113348: F, t113356: F, t113337: F, t113343: F, t113354: F, t99346: F, t99368: F, t99383: F, t113372: F, t113364: F, t113368: F, t113371: F, t113376: F, t113379: F, t113383: F, t113386: F, t113389: F, t113394: F, t113398: F) -> (F, F) {
    let t114366 = t113329 / 18.0;
    let t114367 = 2.0 / 9.0 * t113332;
    let t114370 = 4.0 / 81.0 * t113339;
    let t114372 = t113346 / 18.0;
    let t114373 = t113348 / 27.0;
    let t114375 = t113356 / 27.0;
    let t114377 = -t114366 - t114367 - t113337 - 2.0 / 27.0 * t99346 + t99368 / 27.0 - t114370 + 2.0 / 3.0 * t113343 - t114372 - t114373 + 5.0 / 16.0 * t113354 - t114375 + 2.0 / 27.0 * t99383;
    let t114384 = 4.0 / 27.0 * t113372;
    let t114392 = -t113364 / 9.0 - 2.0 * t113368 + t113371 / 3.0 + t114384 + 2.0 / 3.0 * t113376 + 4.0 / 27.0 * t113379 - t113383 / 9.0 + t113386 / 27.0 + 4.0 / 9.0 * t113389 - t113394 / 3.0 - t113398 / 3.0;
    (t114377, t114392)
}

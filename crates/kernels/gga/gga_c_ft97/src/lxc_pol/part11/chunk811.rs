//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 811/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk811<F: Float>(t37303: F, t37308: F, t37313: F, t37317: F, t37322: F, t37326: F, t37328: F, t37330: F, t37332: F, t37334: F, t37336: F, t37340: F, t37343: F, t37347: F, t37360: F, t37365: F, t37368: F, t37372: F, t37376: F, t37379: F, t37382: F, t37385: F, t37394: F, t37399: F, t37403: F, t37410: F, t37413: F, t37418: F, t37421: F, t37424: F) -> (F, F) {
    let t38418 = 8.0 / 3.0 * t37303 + 40.0 / 27.0 * t37308 - 20.0 / 9.0 * t37313 - 12.0 * t37317 + 8.0 * t37322 + 4.0 / 3.0 * t37326 - 8.0 / 3.0 * t37328 + 8.0 / 9.0 * t37330 - 8.0 / 9.0 * t37332 + 16.0 / 9.0 * t37334 - 4.0 / 3.0 * t37336 + 8.0 * t37340 - 8.0 / 9.0 * t37343 - 16.0 / 27.0 * t37347 - 80.0 / 81.0 * t37360;
    let t38435 = -2.0 / 3.0 * t37365 + 8.0 / 3.0 * t37368 + 8.0 * t37372 + 2.0 * t37376 - 16.0 / 9.0 * t37379 + 112.0 / 81.0 * t37382 + 16.0 / 9.0 * t37385 - t37394 / 3.0 - 36.0 * t37399 + 40.0 / 81.0 * t37403 + 40.0 / 9.0 * t37410 + 112.0 / 27.0 * t37413 + 6.0 * t37418 + 16.0 / 3.0 * t37421 + 4.0 / 3.0 * t37424;
    (t38418, t38435)
}

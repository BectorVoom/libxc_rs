//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 913/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk913<F: Float>(t27263: F, t574: F, t605: F, t1391: F, t2185: F, t3450: F, t3052: F, t569: F, t1882: F, t6649: F, t6641: F, t379: F, t6725: F, t1901: F, t23532: F, t27239: F, t27242: F, t27246: F, t27249: F, t27253: F, t27257: F, t27260: F, t3281: F, t446: F) -> (F, F, F, F, F, F, F) {
    let t27265 = t574 * t605 * t27263;
    let t27269 = t2185 * t1391 * t3450;
    let t27273 = t569 * t1391 * t3052;
    let t27276 = t1882 * t6649;
    let t27278 = t1882 * t6641;
    let t27281 = t569 * t6725 * t379;
    let t27285 = -2.0 / 9.0 * t1901 * t27239 - 2.0 / 9.0 * t1901 * t27242 + t1901 * t27246 / 9.0 + t1901 * t27249 / 9.0 - 2.0 / 9.0 * t1901 * t27253 - t1901 * t27257 / 9.0 - t446 * t27260 / 3.0 + t446 * t27265 / 3.0 + 2.0 / 3.0 * t446 * t27269 - 2.0 / 9.0 * t3281 * t27273 + t27276 / 27.0 - t27278 / 9.0 - t446 * t27281 / 9.0 - 2.0 / 9.0 * t23532;
    (t27265, t27269, t27273, t27276, t27278, t27281, t27285)
}

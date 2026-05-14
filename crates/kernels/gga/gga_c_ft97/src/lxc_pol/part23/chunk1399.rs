//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1399/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1399<F: Float>(t113253: F, t113270: F, t113273: F, t114341: F, t114346: F, t114355: F, t126832: F, t126835: F, t126839: F, t126844: F, t126849: F, t113298: F, t114364: F, t114366: F, t114367: F, t114370: F, t114372: F, t126854: F, t126857: F, t126861: F, t126864: F, t126867: F, t126870: F) -> (F, F) {
    let t128203 = -t114341 + 8.0 / 81.0 * t113253 + t114346 + 4.0 / 27.0 * t113270 + t126832 / 3.0 + 10.0 / 81.0 * t126835 - 8.0 / 27.0 * t126839 + t126844 / 12.0 + t126849 / 3.0 - 4.0 / 27.0 * t113273 - t114355;
    let t128211 = 2.0 / 27.0 * t113298 + 4.0 / 9.0 * t126854 + 2.0 / 3.0 * t126857 - 4.0 / 9.0 * t126861 + 8.0 / 9.0 * t126864 - t114364 - t114366 - t114367 - t114370 - t114372 + t126867 / 24.0 - t126870 / 36.0;
    (t128203, t128211)
}

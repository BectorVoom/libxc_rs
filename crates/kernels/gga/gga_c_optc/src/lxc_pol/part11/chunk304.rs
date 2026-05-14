//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 304/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk304<F: Float>(t127: F, t1271: F, t5: F, t675: F, t116: F, t1256: F, t627: F, t1273: F, t696: F, t1278: F, t673: F, t684: F, t686: F, t695: F, t703: F, t705: F) -> (F, F, F, F) {
    let t1286 = t5 * t1271 * t127;
    let t1287 = t675 * t1286;
    let t1290 = t116 * t1256;
    let t1291 = t627 * t1290;
    let t1294 = t696 * t1273;
    let t1299 = -0.86931614897887578546e-1 * t673 * t1287 - t684 - 0.17386322979577515709e0 * t686 * t1291 - 0.15114211337509259186e-1 * t695 * t1294 - t703 - 0.30228422675018518372e-1 * t705 * t1278;
    (t1287, t1291, t1294, t1299)
}

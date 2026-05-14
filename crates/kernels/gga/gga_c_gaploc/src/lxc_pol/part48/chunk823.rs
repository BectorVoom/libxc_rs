//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 823/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk823<F: Float>(t10612: F, t13445: F, t13446: F, t13450: F, t13453: F, t13458: F, t1580: F, t1599: F, t1641: F, t193: F, t41672: F, t44405: F, t44601: F, t44609: F, t46080: F, t46118: F, t46119: F, t46125: F, t46127: F, t46128: F, t46129: F, t46131: F, t46138: F, t4820: F, t524: F, t531: F, t541: F, t557: F, t568: F, t569: F, t574: F, t597: F, t600: F, t6717: F, t6824: F, t6914: F, t8248: F) -> (F,) {
    let t46162 = -0.12423108009070322895e3 * t6914 * t6717 * t46080 - t46118 + 0.89376224879626066676e-1 * t46119 - t46125 + t46127 - t46128 - t46129 - 0.15337170381568299871e1 * t41672 - t46131 + 0.23833659967900284447e0 * t8248 * t10612 - 0.15889106645266856298e0 * t6824 * t4820 * t44405 - 0.95857314884801874192e0 * t46138 - 0.35750489951850426669e0 * t1599 * t13453 - 0.35750489951850426669e0 * t557 * t531 * t44609 + 0.23833659967900284446e0 * t13446 * t541 + 0.23005755572352449806e1 * t1580 * t13450 + 0.23005755572352449806e1 * t597 * t568 * t600 * t44601 - 0.23005755572352449806e1 * t1641 * t13458 - 0.23005755572352449806e1 * t574 * t568 * t569 * t44601 + 0.35750489951850426669e0 * t524 * t13445 * t193;
    (t46162,)
}

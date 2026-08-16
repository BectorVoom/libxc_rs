//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 957/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk957<F: Float>(t20865: F, t8392: F, t20869: F, t1882: F, t20942: F, t20935: F, t20698: F, t20755: F, t20899: F, t20720: F, t1526: F, t20514: F, t7705: F) -> (F, F, F, F, F, F, F, F, F) {
    let t78438 = t8392 * t20865;
    let t78565 = t8392 * t20869;
    let t78573 = t1882 * t20942;
    let t78584 = t8392 * t20935;
    let t78601 = t1882 * t20698;
    let t78603 = t8392 * t20755;
    let t78605 = t1882 * t20899;
    let t78618 = t1882 * t20720;
    let t78650 = t1526 * t7705 * t20514;
    (t78438, t78565, t78573, t78584, t78601, t78603, t78605, t78618, t78650)
}

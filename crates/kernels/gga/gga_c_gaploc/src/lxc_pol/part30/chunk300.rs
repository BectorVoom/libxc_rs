//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 300/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk300<F: Float>(t110: F, t78: F, t14: F, t85: F, t178: F, t90: F, t112: F, t341: F, t1094: F, t386: F, t1121: F, t72: F) -> (F, F, F, F, F, F) {
    let t1275 = t78 * t110;
    let t1279 = t85 * t14;
    let t1286 = t178 * t90;
    let t1287 = t341 * t112;
    let t1293 = t386 * t1094;
    let t1297 = t72 * t1121;
    (t1275, t1279, t1286, t1287, t1293, t1297)
}

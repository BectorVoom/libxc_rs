//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 293/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk293<F: Float>(t110: F, t78: F, t14: F, t85: F, t178: F, t90: F, t112: F, t341: F, t1094: F, t386: F, t1121: F, t72: F, t400: F, t109: F, t111: F, t1189: F, t427: F, t436: F, t437: F, t441: F, t75: F) -> (F, F, F, F) {
    let t1275 = t78 * t110;
    let t1279 = t85 * t14;
    let t1286 = t178 * t90;
    let t1287 = t341 * t112;
    let t1293 = t386 * t1094;
    let t1297 = t72 * t1121;
    let t1301 = t400 * t400;
    let t1305 = -0.43802864444444444443e-3 * t109 * t1275 * t112 - 0.2e-22 * t436 * t1279 * t112 - 0.26281718666666666666e-2 * t109 * t427 * t441 + 0.19711288999999999999e-2 * t1286 * t1287 + 0.19711288999999999999e-2 * t436 * t437 * t441 + 0.39422577999999999998e-2 * t109 * t111 * t1293 - 0.19711288999999999999e-2 * t109 * t111 * t1297 - 4.0 * t1301 - 4.0 * t75 * t1189;
    (t1275, t1279, t1286, t1305)
}

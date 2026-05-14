//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1135/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1135<F: Float>(t109442: F, t2347: F, t6837: F, t1882: F, t27480: F, t1434: F, t2399: F, t6891: F, t42109: F, t6119: F, t27894: F, t5999: F, t24237: F, t28027: F, t28043: F, t1173: F, t6061: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t109443 = t109442 / 3.0;
    let t109448 = t6837 * t2347;
    let t109469 = t1882 * t27480;
    let t109470 = 4.0 * t109469;
    let t109476 = t1434 * t2399 * t6891;
    let t109481 = t42109 * t6119;
    let t109501 = t27894 * t5999 / 9.0;
    let t109533 = 2.0 / 27.0 * t24237 * t28027;
    let t109535 = 2.0 / 27.0 * t24237 * t28043;
    let t109536 = t6061 * t1173;
    (t109443, t109448, t109469, t109470, t109476, t109481, t109501, t109533, t109535, t109536)
}

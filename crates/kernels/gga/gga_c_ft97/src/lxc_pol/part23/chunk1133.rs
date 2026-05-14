//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1133/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1133<F: Float>(t109314: F, t27500: F, t1424: F, t42123: F, t24447: F, t27822: F, t458: F, t27798: F, t96925: F, t11176: F, t1433: F, t27807: F, t92: F, t97168: F, t743: F, t9568: F) -> (F, F, F, F, F, F, F, F, F) {
    let t109316 = 0.56749874115226337448e-2 * t27500 * t109314;
    let t109335 = t42123 * t1424;
    let t109356 = t24447 * t458 * t27822;
    let t109357 = t109356 / 4.0;
    let t109358 = t96925 * t27798;
    let t109359 = t109358 / 3.0;
    let t109361 = t1433 * t11176 * t27807;
    let t109363 = t97168 * t92;
    let t109377 = t9568 * t743;
    (t109316, t109335, t109356, t109357, t109358, t109359, t109361, t109363, t109377)
}

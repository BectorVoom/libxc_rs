//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1134/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1134<F: Float>(t192: F, t9942: F, t1424: F, t42109: F, t1900: F, t6: F, t734: F, t91: F, t42123: F, t6119: F, t1636: F, t6903: F, t89: F, t27781: F, t375: F, t1434: F, t27743: F, t681: F) -> (F, F, F, F, F, F, F, F) {
    let t109390 = t192 * t9942;
    let t109402 = t42109 * t1424;
    let t109414 = t91 * t734 * t6 * t1900;
    let t109415 = t42123 * t6119;
    let t109434 = t89 * t1636 * t6903;
    let t109437 = t89 * t375 * t27781;
    let t109438 = 2.0 / 3.0 * t109437;
    let t109442 = t1434 * t681 * t27743;
    (t109390, t109402, t109414, t109415, t109434, t109437, t109438, t109442)
}

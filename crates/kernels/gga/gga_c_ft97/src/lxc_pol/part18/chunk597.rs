//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 597/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk597<F: Float>(t1843: F, t376: F, t89: F, t7822: F, t7778: F, t7782: F, t7820: F, t8195: F, t7771: F, t8189: F, t1851: F, t480: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8430 = t89 * t376 * t1843;
    let t8437 = 2.0 / 9.0 * t7822;
    let t8444 = t7778 / 9.0;
    let t8446 = 2.0 / 27.0 * t7782;
    let t8449 = 2.0 / 9.0 * t7820;
    let t8452 = t8195 / 3.0;
    let t8454 = 2.0 / 3.0 * t7771;
    let t8455 = 28.0 / 81.0 * t8189;
    let t8466 = t480 * t1851;
    (t8430, t8437, t8444, t8446, t8449, t8452, t8454, t8455, t8466)
}

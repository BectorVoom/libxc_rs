//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1291/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1291<F: Float>(t30239: F, t446: F, t558: F, t9432: F, t1882: F, t30200: F, t2185: F, t23657: F, t4778: F, t5900: F, t27157: F, t27158: F, t27165: F, t574: F, t119308: F, t28: F, t89: F) -> (F, F, F, F, F, F) {
    let t120059 = t446 * t9432 * t30239 * t558;
    let t120061 = t1882 * t30200;
    let t120062 = 2.0 / 27.0 * t120061;
    let t120066 = t23657 * t2185 * t5900 * t4778 * t558;
    let t120070 = t27157 * t574 * t27165 * t27158;
    let t120074 = t89 * t28 * t119308 * t558;
    (t120059, t120061, t120062, t120066, t120070, t120074)
}

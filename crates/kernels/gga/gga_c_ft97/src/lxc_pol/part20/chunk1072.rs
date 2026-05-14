//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1072/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1072<F: Float>(t1434: F, t2399: F, t6887: F, t14075: F, t24519: F, t446: F, t9744: F, t6884: F, t96982: F, t193: F, t2459: F, t27882: F, t89: F, t6899: F, t2413: F, t24437: F, t24438: F, t6878: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t108278 = t1434 * t2399 * t6887;
    let t108279 = 4.0 / 9.0 * t108278;
    let t108280 = t24519 * t14075;
    let t108282 = t446 * t9744 * t108280;
    let t108284 = t96982 * t6884;
    let t108285 = 2.0 / 27.0 * t108284;
    let t108288 = t89 * t193 * t27882 * t2459;
    let t108291 = t89 * t2399 * t6899;
    let t108292 = 8.0 / 9.0 * t108291;
    let t108295 = t24437 * t24438 * t6878 * t2413;
    (t108278, t108279, t108280, t108282, t108284, t108285, t108288, t108291, t108292, t108295)
}

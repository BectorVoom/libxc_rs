//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1076/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1076<F: Float>(t108353: F, t27828: F, t681: F, t89: F, t1131: F, t193: F, t96808: F, t14133: F, t42500: F, t6118: F, t6119: F, t2405: F, t6852: F, t41879: F, t446: F, t108218: F, t9770: F) -> (F, F, F, F, F, F, F, F) {
    let t108354 = 4.0 / 3.0 * t108353;
    let t108356 = t89 * t681 * t27828;
    let t108357 = 4.0 / 3.0 * t108356;
    let t108360 = t89 * t193 * t96808 * t1131;
    let t108364 = t6118 * t42500 * t6119 * t14133;
    let t108366 = t6852 * t2405;
    let t108368 = t446 * t41879 * t108366;
    let t108371 = t446 * t9770 * t108218;
    (t108354, t108356, t108357, t108360, t108364, t108366, t108368, t108371)
}

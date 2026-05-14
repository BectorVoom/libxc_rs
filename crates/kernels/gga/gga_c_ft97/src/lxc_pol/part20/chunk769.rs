//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 769/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk769<F: Float>(t6140: F, t681: F, t89: F, t1424: F, t2514: F, t743: F, t193: F, t6109: F, t2373: F, t9942: F, t1434: F, t24395: F, t6128: F, t2347: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t24499 = t681 * t6140;
    let t24500 = t89 * t24499;
    let t24502 = t1424 * t2514;
    let t24503 = t743 * t24502;
    let t24505 = t6109 * t193 * t24503;
    let t24507 = t1424 * t2373;
    let t24508 = t9942 * t24507;
    let t24510 = t1434 * t193 * t24508;
    let t24512 = t743 * t24395;
    let t24514 = t1434 * t193 * t24512;
    let t24517 = t1434 * t681 * t6128;
    let t24519 = t1424 * t2347;
    (t24500, t24503, t24505, t24507, t24508, t24510, t24512, t24514, t24517, t24519)
}

//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 815/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk815<F: Float>(t91: F, t9890: F, t26: F, t6109: F, t6111: F, t681: F, t1434: F, t6124: F, t1439: F, t1636: F, t89: F, t375: F, t6144: F, t6140: F, t6128: F, t1424: F, t2347: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t24447 = t91 * t9890;
    let t24448 = t24447 * t26;
    let t24455 = t6109 * t681 * t6111;
    let t24470 = t1434 * t681 * t6124;
    let t24482 = t89 * t1636 * t1439;
    let t24483 = 4.0 / 9.0 * t24482;
    let t24485 = t89 * t375 * t6144;
    let t24499 = t681 * t6140;
    let t24500 = t89 * t24499;
    let t24517 = t1434 * t681 * t6128;
    let t24519 = t1424 * t2347;
    (t24447, t24448, t24455, t24470, t24482, t24483, t24485, t24500, t24517, t24519)
}

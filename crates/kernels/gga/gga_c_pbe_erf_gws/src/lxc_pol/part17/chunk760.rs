//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 760/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk760<F: Float>(t6505: F, t935: F, t2323: F, t2327: F, t2206: F, t2212: F, t2285: F, t2289: F, t3205: F, t336: F, t2182: F, t343: F, t2122: F, t337: F, t810: F, t2147: F) -> (F, F, F, F, F, F, F) {
    let t6506 = t6505 * t935;
    let t6508 = t2323 * t2327;
    let t6510 = t2206 * t2212;
    let t6517 = t2289 * t2285;
    let t6523 = t3205 * t336;
    let t6524 = t343 * t2182;
    let t6534 = t337 * t2122 * t810;
    let t6535 = t2147 * t6534;
    (t6506, t6508, t6510, t6517, t6523, t6524, t6535)
}

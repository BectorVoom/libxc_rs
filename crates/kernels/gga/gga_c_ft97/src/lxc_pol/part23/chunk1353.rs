//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1353/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1353<F: Float>(t31619: F, t6308: F, t681: F, t126946: F, t4255: F, t99528: F, t99529: F, t113459: F, t1234: F, t2360: F, t3886: F, t1091: F, t28776: F, t31631: F, t89: F, t193: F, t28835: F, t4129: F) -> (F, F, F, F, F, F, F, F) {
    let t126982 = t6308 * t681 * t31619;
    let t126983 = t126982 / 6.0;
    let t126986 = t99528 * t99529 * t126946 * t4255;
    let t126991 = t99528 * t113459 * t1234 * t2360 * t3886;
    let t126995 = t99528 * t99529 * t1091 * t28776;
    let t126998 = t89 * t681 * t31631;
    let t126999 = 2.0 / 3.0 * t126998;
    let t127002 = t89 * t193 * t28835 * t4129;
    (t126982, t126983, t126986, t126991, t126995, t126998, t126999, t127002)
}

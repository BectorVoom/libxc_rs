//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1379/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1379<F: Float>(t127728: F, t27: F, t799: F, t89: F, t193: F, t24964: F, t5299: F, t19240: F, t6222: F, t31635: F, t681: F, t1882: F, t31371: F, t1486: F, t852: F, t10683: F, t4129: F, t446: F, t7036: F) -> (F, F, F, F, F, F, F, F, F) {
    let t127731 = t89 * t27 * t799 * t127728;
    let t127735 = t89 * t193 * t24964 * t5299;
    let t127739 = t89 * t193 * t6222 * t19240;
    let t127742 = t89 * t681 * t31635;
    let t127743 = 4.0 / 3.0 * t127742;
    let t127744 = t1882 * t31371;
    let t127745 = 2.0 / 27.0 * t127744;
    let t127748 = t1486 * t193 * t852 * t127728;
    let t127752 = t446 * t10683 * t7036 * t4129;
    (t127731, t127735, t127739, t127742, t127743, t127744, t127745, t127748, t127752)
}

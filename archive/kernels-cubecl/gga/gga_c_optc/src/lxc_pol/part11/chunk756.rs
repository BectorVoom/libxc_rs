//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 756/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk756<F: Float>(t11007: F, t862: F, t330: F, t8113: F, t1388: F, t7878: F, t893: F, t1384: F, t2619: F, t874: F, t1392: F, t2693: F) -> (F, F, F, F, F, F) {
    let t11008 = t862 * t11007;
    let t11018 = t330 * t8113;
    let t11073 = t7878 * t1388;
    let t11074 = t893 * t11073;
    let t11110 = t2619 * t1384;
    let t11111 = t874 * t11110;
    let t11130 = t1392 * t2693;
    (t11008, t11018, t11073, t11074, t11111, t11130)
}

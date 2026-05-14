//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1009/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1009<F: Float>(t30963: F, t6118: F, t4965: F, t6119: F, t9744: F, t1131: F, t1154: F, t2574: F, t24437: F, t24531: F, t4917: F, t2354: F, t446: F, t1091: F, t24438: F, t6878: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30964 = t6118 * t30963;
    let t30967 = t9744 * t6119 * t4965;
    let t30968 = t6118 * t30967;
    let t30970 = t1154 * t1131;
    let t30972 = t2574 * t6119 * t30970;
    let t30973 = t24437 * t30972;
    let t30974 = t24531 * t4917;
    let t30975 = t2354 * t30974;
    let t30976 = t446 * t30975;
    let t30979 = t24438 * t6878 * t1091;
    (t30964, t30967, t30968, t30970, t30972, t30973, t30975, t30976, t30979)
}

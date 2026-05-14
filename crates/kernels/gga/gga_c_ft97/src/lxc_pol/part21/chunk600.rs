//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 600/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk600<F: Float>(t12001: F, t3283: F, t103: F, t7800: F, t1570: F, t2266: F, t1557: F, t8654: F, t2253: F, t3642: F, t1736: F, t179: F, t3627: F, t41: F, t70: F, t3618: F, t8675: F) -> (F, F, F, F, F, F, F, F) {
    let t12002 = t12001 * t3283;
    let t12020 = t103 * t7800;
    let t12116 = t2266 * t1570;
    let t12122 = t8654 * t1557;
    let t12132 = 2.0 * t2253 * t3642;
    let t12137 = t1736 * t179;
    let t12143 = t41 * t3627 * t70;
    let t12162 = 4.0 / 9.0 * t8675 * t3618;
    (t12002, t12020, t12116, t12122, t12132, t12137, t12143, t12162)
}

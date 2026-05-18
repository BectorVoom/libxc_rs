//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 652/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk652<F: Float>(t143: F, t7800: F, t7765: F, t1555: F, t89: F, t1882: F, t1971: F, t1546: F, t1975: F, t1636: F, t559: F, t2076: F, t375: F) -> (F, F, F, F, F, F, F) {
    let t9054 = t143 * t7800;
    let t9055 = t9054 * t7765;
    let t9057 = t89 * t1555 * t9055;
    let t9059 = t1882 * t1971;
    let t9062 = t89 * t1546 * t1975;
    let t9065 = t89 * t1636 * t559;
    let t9068 = t89 * t375 * t2076;
    (t9054, t9055, t9057, t9059, t9062, t9065, t9068)
}

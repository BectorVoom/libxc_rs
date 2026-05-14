//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 183/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk183<F: Float>(t146: F, t574: F, t155: F, t95: F, t161: F, t149: F) -> (F, F, F, F) {
    let t575 = t146 * t574;
    let t578 = 7.0 / 288.0 * t575 * t95 * t155;
    let t579 = t95 * t161;
    let t580 = t149 * t579;
    (t575, t578, t579, t580)
}

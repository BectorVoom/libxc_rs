//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 179/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk179<F: Float>(t191: F, t633: F, t203: F, t457: F, t201: F, t197: F, t122: F, t188: F) -> (F, F, F, F, F) {
    let t634 = t633 * t191;
    let t635 = t203 * t457;
    let t636 = t201 * t635;
    let t637 = t197 * t636;
    let t640 = t122 * t188;
    (t634, t635, t636, t637, t640)
}

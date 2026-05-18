//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 155/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk155<F: Float>(t71: F, t57: F, t46: F, t58: F, t48: F, t51: F) -> (F, F, F, F, F) {
    let t470 = t71 * t71;
    let t471 = F::new(1.0) / t470;
    let t472 = t57 * t471;
    let t474 = F::new(1.0) / t58 * t46;
    let t475 = t48 * t51;
    (t470, t471, t472, t474, t475)
}

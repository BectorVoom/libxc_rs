//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 154/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk154<F: Float>(t71: F, t57: F, t46: F, t58: F, t48: F, t51: F) -> (F, F, F, F, F) {
    let t470 = t71 * t71;
    let t471 = F::cast_from(1.0_f64) / t470;
    let t472 = t57 * t471;
    let t474 = F::cast_from(1.0_f64) / t58 * t46;
    let t475 = t48 * t51;
    (t470, t471, t472, t474, t475)
}

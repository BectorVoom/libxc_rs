//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1869/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1869<F: Float>(t232: F, t46693: F, t6605: F, t815: F, t2628: F, t58345: F, t2632: F, t47262: F, t22996: F, t6590: F, t25130: F, t828: F, t9627: F) -> (F, F, F, F) {
    let t87495 = t6605 * t815 * t46693 * t232;
    let t87498 = t6605 * t2628 * t58345;
    let t87502 = t6605 * t2628 * t47262 * t2632;
    let t87504 = t6590 * t22996;
    let t87507 = t87504 * t25130 * t9627 * t828;
    (t87495, t87498, t87502, t87507)
}

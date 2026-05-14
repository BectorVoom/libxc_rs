//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 940/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk940<F: Float>(t12599: F, t12607: F, t12611: F, t12620: F, t576: F, t3916: F, t699: F, t3914: F, t972: F, t12044: F, t12045: F, t12046: F, t12054: F, t12152: F, t12154: F, t12155: F, t12156: F, t12158: F, t12161: F, t12162: F, t12192: F, t12281: F, t12592: F, t2469: F) -> (F, F, F, F, F) {
    let t12622 = t12599 + t12607 + t12611 + t12620;
    let t12623 = t576 * t12622;
    let t12624 = t699 * t3916;
    let t12625 = t3914 * t972;
    let t12628 = 2.0 * t12625 * t2469 - t12044 + t12045 + t12046 + t12054 - t12152 - t12154 - t12155 - t12156 + t12158 + t12161 - t12162 + t12192 + t12281 - t12592;
    (t12622, t12623, t12624, t12625, t12628)
}

//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1069/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1069<F: Float>(t545: F, t6798: F, t83: F, t16626: F, t16631: F, t16701: F, t16873: F, t16875: F, t19776: F, t19778: F, t19796: F, t19798: F, t19799: F, t19804: F, t19806: F, t19807: F, t19823: F, t19825: F) -> (F, F) {
    let t20325 = t83 * t6798 * t545;
    let t20326 = 3.0 * t20325;
    let t20327 = t19776 + t19778 + t16626 - t16631 - t19796 - t19798 - t19799 + t19804 - t19806 + t19807 + t16873 + t16701 - t19823 + t19825 + t20326 - t16875;
    (t20326, t20327)
}

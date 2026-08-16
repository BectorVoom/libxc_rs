//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1878/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1878<F: Float>(t28159: F, t6897: F, t794: F, t19763: F, t1992: F, t6976: F, t19739: F, t22633: F, t3807: F, t28131: F, t81159: F, t552: F, t6434: F) -> (F, F, F, F, F) {
    let t97111 = t6897 * t794 * t28159;
    let t97114 = t1992 * t6976 * t19763;
    let t97119 = t22633 * t6976 * t19739 * t3807;
    let t97124 = t81159 * t28131;
    let t97126 = t552 * t6434;
    (t97111, t97114, t97119, t97124, t97126)
}

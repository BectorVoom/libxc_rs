//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1810/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1810<F: Float>(t23034: F, t6546: F, t23037: F, t131: F, t845: F, t23159: F, t23168: F, t23177: F, t6579: F, t23143: F, t6649: F, t22999: F) -> (F, F, F, F, F, F, F) {
    let t81979 = t6546 * t23034;
    let t81980 = t81979 * t23037;
    let t81982 = t845 * t131;
    let t81989 = t23168 * t23159;
    let t82005 = t6579 * t23177;
    let t82011 = t23143 * t6649;
    let t82013 = t6579 * t22999;
    (t81979, t81980, t81982, t81989, t82005, t82011, t82013)
}

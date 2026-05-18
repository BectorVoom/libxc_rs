//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 747/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk747<F: Float>(t1692: F, t1734: F, t179: F, t1753: F, t600: F, t164: F, t1732: F, t2590: F) -> (F, F, F, F, F) {
    let t5236 = t179 * t1734 * t1692;
    let t5239 = t1753 * t600;
    let t5240 = t5239 * t164;
    let t5241 = t179 * t5240;
    let t5244 = t2590 * t1732;
    (t5236, t5239, t5240, t5241, t5244)
}

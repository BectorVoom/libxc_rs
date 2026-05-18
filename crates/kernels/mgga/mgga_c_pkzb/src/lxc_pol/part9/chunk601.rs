//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 601/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk601<F: Float>(t2593: F, t600: F, t179: F, t1037: F, t1727: F, t1034: F, t164: F) -> (F, F, F, F) {
    let t2594 = t2593 * t600;
    let t2595 = t179 * t2594;
    let t2598 = t1727 * t1037;
    let t2600 = t1034 * t164;
    (t2594, t2595, t2598, t2600)
}

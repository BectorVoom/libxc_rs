//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 750/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk750<F: Float>(t5269: F, t581: F, t164: F, t1719: F, t179: F, t568: F, t1731: F, t1773: F, t1730: F) -> (F, F, F, F) {
    let t5270 = t581 * t5269;
    let t5273 = t1719 * t164;
    let t5275 = t179 * t5273 * t568;
    let t5278 = t1731 * t1773;
    let t5279 = t1730 * t5278;
    (t5270, t5275, t5278, t5279)
}

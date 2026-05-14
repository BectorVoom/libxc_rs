//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1003/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1003<F: Float>(t237: F, t5762: F, t1847: F, t1854: F, t5775: F, t659: F, t11817: F, t204: F, t208: F) -> (F, F, F, F) {
    let t17312 = t237 * t5762;
    let t17326 = t1847 * t1854;
    let t17329 = t659 * t5775;
    let t17348 = t204 * t11817 * t208;
    (t17312, t17326, t17329, t17348)
}

//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 958/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk958<F: Float>(t284: F, t8753: F, t3419: F, t847: F, t1371: F, t2306: F, t2291: F, t3422: F, t2314: F, t3418: F, t1370: F, t6669: F) -> (F, F, F, F, F, F, F, F) {
    let t8754 = t8753 * t284;
    let t8760 = t3419 * t847;
    let t8763 = t1371 * t2306;
    let t8766 = t3422 * t2291;
    let t8769 = t3418 * t2314;
    let t8770 = t8769 * t847;
    let t8773 = t3422 * t2306;
    let t8776 = t1370 * t6669;
    (t8754, t8760, t8763, t8766, t8769, t8770, t8773, t8776)
}

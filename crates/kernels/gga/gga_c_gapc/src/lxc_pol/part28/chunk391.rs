//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 391/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk391<F: Float>(t136: F, t1845: F, t191: F, t617: F, t636: F, t1044: F, t147: F, t19: F, t648: F, t118: F, t1388: F, t129: F) -> (F, F, F, F, F, F, F) {
    let t1846 = t1845 * t136;
    let t1847 = t1846 * t191;
    let t1850 = t617 * t636;
    let t1854 = t1044 * t19 * t147;
    let t1855 = t1854 * t648;
    let t1860 = t1388 * t118;
    let t1861 = t1860 * t129;
    (t1846, t1847, t1850, t1854, t1855, t1860, t1861)
}

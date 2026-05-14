//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 734/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk734<F: Float>(t7140: F, t7196: F, t7249: F, t7320: F, t1959: F, t952: F, t2728: F, t841: F, t2208: F, t977: F, t1402: F, t2576: F, t1: F, t7275: F, t787: F, t161: F, t165: F, t1835: F) -> (F, F, F, F, F, F, F, F) {
    let t7322 = t7140 + t7196 + t7249 + t7320;
    let t7324 = t952 * t1959;
    let t7329 = t2728 * t841;
    let t7332 = t977 * t2208;
    let t7336 = t1402 * t2576;
    let t7339 = t7275 * t1;
    let t7340 = t787 * t7339;
    let t7344 = t161 * t165 * t1835;
    (t7322, t7324, t7329, t7332, t7336, t7339, t7340, t7344)
}

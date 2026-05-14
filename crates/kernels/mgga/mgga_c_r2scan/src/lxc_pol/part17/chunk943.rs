//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 943/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk943<F: Float>(t3090: F, t560: F, t3232: F, t6897: F, t2333: F, t9563: F, t19026: F, t3245: F, t1275: F, t2924: F, t818: F, t9638: F, t10533: F, t856: F, t352: F, t9769: F) -> (F, F, F, F, F, F, F, F) {
    let t31064 = t3090 * t560;
    let t31393 = t3232 * t6897;
    let t31498 = t9563 * t2333;
    let t31510 = t3245 * t19026;
    let t31689 = t2924 * t1275;
    let t31764 = t9638 * t818;
    let t35213 = t10533 * t856;
    let t35220 = t352 * t9769;
    (t31064, t31393, t31498, t31510, t31689, t31764, t35213, t35220)
}

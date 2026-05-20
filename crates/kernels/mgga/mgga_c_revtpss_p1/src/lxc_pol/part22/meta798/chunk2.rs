//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2898/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2898<F: Float>(t1413: F, t46826: F, t46835: F, t1376: F, t40769: F, t10111: F, t1386: F, t9720: F, t1390: F, t1399: F, t685: F, t9970: F, t9976: F) -> (F, F, F, F, F) {
    let t46837 = t46835 * t1413 * t46826;
    let t46840 = F::cast_from(0.70398079132139197745e-2_f64) * t40769 * t1376;
    let t46856 = t10111 * t1386 * t9720;
    let t46859 = t46856 * t1390 * t685 * t1399;
    let t46861 = t9976 * t9970;
    (t46837, t46840, t46856, t46859, t46861)
}

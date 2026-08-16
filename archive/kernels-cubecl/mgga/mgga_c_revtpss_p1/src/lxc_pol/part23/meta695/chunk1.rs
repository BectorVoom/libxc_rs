//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2443/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2443<F: Float>(t1412: F, t9794: F, t40609: F, t4062: F, t3994: F, t40763: F, t9793: F, t2735: F, t9792: F, t1376: F, t40769: F, t10111: F, t1386: F, t9720: F) -> (F, F, F, F, F, F) {
    let t46825 = t9794 * t1412;
    let t46831 = F::cast_from(0.63807336860547134325e-3_f64) * t40609 * t4062;
    let t46833 = t9793 * t40763 * t3994;
    let t46835 = t2735 * t9792;
    let t46840 = F::cast_from(0.70398079132139197745e-2_f64) * t40769 * t1376;
    let t46856 = t10111 * t1386 * t9720;
    (t46825, t46831, t46833, t46835, t46840, t46856)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2551/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2551<F: Float>(t1379: F, t40846: F, t550: F, t816: F, t1412: F, t9794: F, t1353: F, t1399: F, t9793: F, t40609: F, t4062: F, t3994: F, t40763: F) -> (F, F, F, F, F, F) {
    let t46824 = F::cast_from(0.12516778469694349359e-1_f64) * t1379 * t40846 * t550 * t816;
    let t46825 = t9794 * t1412;
    let t46826 = t1399 * t1353;
    let t46828 = t9793 * t46825 * t46826;
    let t46831 = F::cast_from(0.63807336860547134325e-3_f64) * t40609 * t4062;
    let t46833 = t9793 * t40763 * t3994;
    (t46824, t46825, t46826, t46828, t46831, t46833)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2893/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2893<F: Float>(t46740: F, t9821: F, t9769: F, t9793: F, t9794: F, t1376: F, t40757: F, t2681: F, t4000: F, t820: F, t4006: F, t10111: F, t1408: F, t9720: F) -> (F, F, F, F, F, F) {
    let t46741 = t46740 * t9821;
    let t46757 = t9793 * t9794 * t9769;
    let t46760 = F::cast_from(0.26776076960158126592e-7_f64) * t40757 * t1376;
    let t46766 = t820 * t4000 * t2681;
    let t46767 = t46766 * t4006;
    let t46784 = t10111 * t1408 * t9720;
    (t46741, t46757, t46760, t46766, t46767, t46784)
}

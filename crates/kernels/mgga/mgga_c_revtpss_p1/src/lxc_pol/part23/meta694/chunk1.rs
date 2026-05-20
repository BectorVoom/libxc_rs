//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2441/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2441<F: Float>(t10111: F, t1408: F, t9720: F, t1353: F, t1414: F, t685: F, t40735: F, t535: F, t235: F, t5744: F, t2453: F, t1389: F, t268: F) -> (F, F, F, F, F, F, F) {
    let t46784 = t10111 * t1408 * t9720;
    let t46786 = t1414 * t685 * t1353;
    let t46787 = t46784 * t46786;
    let t46800 = F::new(455.0) / F::new(243.0) * t40735 * t535;
    let t46801 = t5744 * t235;
    let t46802 = t2453 * t46801;
    let t46808 = t1389 * t268;
    (t46784, t46786, t46787, t46800, t46801, t46802, t46808)
}

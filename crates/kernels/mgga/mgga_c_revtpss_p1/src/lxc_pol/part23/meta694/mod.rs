//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta694 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2440;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2441;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta694<F: Float>(t1401: F, t46722: F, t159: F, t216: F, t4010: F, t1386: F, t2482: F, t2668: F, t1376: F, t40757: F, t2681: F, t4000: F, t820: F, t10111: F, t1408: F, t9720: F, t1353: F, t1414: F, t685: F, t40735: F, t535: F, t235: F, t5744: F, t2453: F, t1389: F, t268: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t46723, t46730, t46740, t46760, t46766) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2440::<F>(t1401, t46722, t159, t216, t4010, t1386, t2482, t2668, t1376, t40757, t2681, t4000, t820);
        let (t46784, t46786, t46787, t46800, t46801, t46802, t46808) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2441::<F>(t10111, t1408, t9720, t1353, t1414, t685, t40735, t535, t235, t5744, t2453, t1389, t268);
    (t46723, t46730, t46740, t46760, t46766, t46784, t46786, t46787, t46800, t46801, t46802, t46808)
}

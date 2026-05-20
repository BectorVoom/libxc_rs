//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta444 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1699;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1700;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1701;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta444<F: Float>(t30: F, t3889: F, t3853: F, t3860: F, t10179: F, t4147: F, t513: F, t9603: F, t3834: F, t2257: F, t1344: F, t3874: F, t39456: F, t9344: F, t9605: F, t9608: F, zeta_threshold: F, t33: F, t516: F, t9615: F, t3842: F, t3351: F, t1348: F, t3881: F, t43744: F, t9357: F, t9617: F, t9620: F, t1343: F, t13656: F, t1448: F, t198: F, t3828: F, t3829: F, t39419: F, t39422: F, t46280: F, t46282: F, t46287: F, t46290: F, t46292: F, t46297: F, t5536: F, t5541: F, t9547: F) -> (F, F, F, F, F, F, F, F) {
        let (t46298, t46303, t46304, t46311, t46317, t46325) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1699::<F>(t30, t3889, t3853, t3860, t10179, t4147, t513, t9603, t3834, t2257, t1344, t3874, t39456, t9344, t9605, t9608, zeta_threshold);
        let (t46329, t46335, t46345) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1700::<F>(t33, t516, t9615, t3842, t3351, t1348, t3881, t43744, t9357, t9617, t9620, t46325, zeta_threshold);
        let t46349 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1701::<F>(t1343, t13656, t1448, t198, t3828, t3829, t3889, t39419, t39422, t46280, t46282, t46287, t46290, t46292, t46297, t46298, t46303, t46304, t46345, t5536, t5541, t9547);
    (t46298, t46303, t46311, t46317, t46329, t46335, t46345, t46349)
}

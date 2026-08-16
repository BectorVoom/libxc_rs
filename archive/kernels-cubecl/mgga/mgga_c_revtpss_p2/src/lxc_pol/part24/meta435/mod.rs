//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta435 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1387;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1388;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta435<F: Float>(t794: F, t9747: F, t2699: F, t3943: F, t136: F, t9941: F, t1386: F, t820: F, t9948: F, t159: F, t216: F, t4010: F, t2482: F, t2668: F, t1376: F, t40757: F, t2681: F, t4000: F, t10111: F, t1408: F, t9720: F, t40735: F, t535: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t46691, t46694, t46716, t46722, t46730) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1387::<F>(t794, t9747, t2699, t3943, t136, t9941, t1386, t820, t9948, t159, t216, t4010);
        let (t46740, t46760, t46766, t46784, t46800) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1388::<F>(t1386, t2482, t2668, t1376, t40757, t2681, t4000, t820, t10111, t1408, t9720, t40735, t535);
    (t46691, t46694, t46716, t46722, t46730, t46740, t46760, t46766, t46784, t46800)
}

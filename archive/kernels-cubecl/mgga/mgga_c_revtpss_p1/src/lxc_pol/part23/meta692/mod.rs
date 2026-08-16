//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta692 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2436;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2437;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta692<F: Float>(t10139: F, t1398: F, t281: F, t543: F, t624: F, t1433: F, t39545: F, t546: F, t685: F, t39552: F, t557: F, t1438: F, t40317: F, t1419: F, t9990: F, t4089: F, t40921: F, t1408: F, t820: F, t9948: F, t1416: F, t240: F, t9991: F, t3995: F, t40488: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t46505, t46515, t46518, t46526) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2436::<F>(t10139, t1398, t281, t543, t624, t1433, t39545, t546, t685, t39552, t557, t1438, t40317);
        let (t46554, t46570, t46595, t46596, t46609, t46620) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2437::<F>(t1419, t9990, t4089, t40921, t1408, t820, t9948, t1416, t240, t9991, t3995, t40488);
    (t46505, t46515, t46518, t46526, t46554, t46570, t46595, t46596, t46609, t46620)
}

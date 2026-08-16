//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta692 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2436;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2437;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta692(t10139: f64, t1398: f64, t281: f64, t543: f64, t624: f64, t1433: f64, t39545: f64, t546: f64, t685: f64, t39552: f64, t557: f64, t1438: f64, t40317: f64, t1419: f64, t9990: f64, t4089: f64, t40921: f64, t1408: f64, t820: f64, t9948: f64, t1416: f64, t240: f64, t9991: f64, t3995: f64, t40488: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46505, t46515, t46518, t46526) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2436(t10139, t1398, t281, t543, t624, t1433, t39545, t546, t685, t39552, t557, t1438, t40317);
        let (t46554, t46570, t46595, t46596, t46609, t46620) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2437(t1419, t9990, t4089, t40921, t1408, t820, t9948, t1416, t240, t9991, t3995, t40488);
    (t46505, t46515, t46518, t46526, t46554, t46570, t46595, t46596, t46609, t46620)
}

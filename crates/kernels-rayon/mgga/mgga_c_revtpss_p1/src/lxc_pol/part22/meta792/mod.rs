//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta792 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2885;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2886;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta792(t544: f64, t9989: f64, t4003: f64, t215: f64, t268: f64, t4056: f64, t4101: f64, t543: f64, t10013: f64, t2453: f64, t10142: f64, t136: f64, t2457: f64, t3964: f64, t4066: f64, t10139: f64, t1398: f64, t281: f64, t624: f64, t3923: f64, t68: f64, t1433: f64, t39545: f64, t546: f64, t685: f64, t39552: f64, t557: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46475, t46478, t46490, t46495, t46496, t46500) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2885(t544, t9989, t4003, t215, t268, t4056, t4101, t543, t10013, t2453, t10142, t136, t2457, t3964, t4066);
        let (t46505, t46507, t46510, t46515, t46518) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2886(t10139, t1398, t281, t543, t624, t3923, t68, t1433, t39545, t546, t685, t39552, t557);
    (t46475, t46478, t46490, t46495, t46496, t46500, t46505, t46507, t46510, t46515, t46518)
}

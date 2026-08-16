//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta792 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2885;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2886;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta792<F: Float>(t544: F, t9989: F, t4003: F, t215: F, t268: F, t4056: F, t4101: F, t543: F, t10013: F, t2453: F, t10142: F, t136: F, t2457: F, t3964: F, t4066: F, t10139: F, t1398: F, t281: F, t624: F, t3923: F, t68: F, t1433: F, t39545: F, t546: F, t685: F, t39552: F, t557: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t46475, t46478, t46490, t46495, t46496, t46500) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2885::<F>(t544, t9989, t4003, t215, t268, t4056, t4101, t543, t10013, t2453, t10142, t136, t2457, t3964, t4066);
        let (t46505, t46507, t46510, t46515, t46518) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2886::<F>(t10139, t1398, t281, t543, t624, t3923, t68, t1433, t39545, t546, t685, t39552, t557);
    (t46475, t46478, t46490, t46495, t46496, t46500, t46505, t46507, t46510, t46515, t46518)
}

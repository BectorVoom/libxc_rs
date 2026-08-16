//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta707 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2534;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2535;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta707<F: Float>(t10022: F, t2782: F, t46422: F, t10013: F, t2453: F, t10142: F, t136: F, t2457: F, t3964: F, t4066: F, t10139: F, t1398: F, t281: F, t543: F, t624: F, t3923: F, t68: F, t1433: F, t39545: F, t546: F, t685: F, t39552: F, t557: F, t10103: F, t1432: F, t2470: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t46493, t46495, t46496, t46500, t46505) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2534::<F>(t10022, t2782, t46422, t10013, t2453, t10142, t136, t2457, t3964, t4066, t10139, t1398, t281, t543, t624);
        let (t46507, t46510, t46515, t46518, t46520) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2535::<F>(t3923, t68, t10139, t281, t543, t1433, t39545, t546, t685, t39552, t557, t10103, t1432, t2470);
    (t46493, t46495, t46496, t46500, t46505, t46507, t46510, t46515, t46518, t46520)
}

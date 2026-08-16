//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta448 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1710;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1711;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta448(t10022: f64, t2782: f64, t46422: f64, t10013: f64, t2453: f64, t10142: f64, t136: f64, t2457: f64, t3964: f64, t4066: f64, t10139: f64, t1398: f64, t281: f64, t543: f64, t624: f64, t3923: f64, t68: f64, t1433: f64, t39545: f64, t546: f64, t685: f64, t39552: f64, t557: f64, t10103: f64, t1432: f64, t2470: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46493, t46496, t46500, t46505) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1710(t10022, t2782, t46422, t10013, t2453, t10142, t136, t2457, t3964, t4066, t10139, t1398, t281, t543, t624);
        let (t46507, t46510, t46515, t46518, t46520) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1711(t3923, t68, t10139, t281, t543, t1433, t39545, t546, t685, t39552, t557, t10103, t1432, t2470);
    (t46493, t46496, t46500, t46505, t46507, t46510, t46515, t46518, t46520)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta449 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1712;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1713;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1714;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta449(t3999: f64, t4066: f64, t1438: f64, t40317: f64, t4056: f64, t543: f64, t10065: f64, t10069: f64, t2782: f64, t4086: f64, t46469: f64, t10084: f64, t4003: f64, t46477: f64, t1437: f64, t4004: f64, t4114: f64, t4118: f64, t46518: f64, t46520: f64, t820: f64, t9891: f64, t9899: f64, t1419: f64, t9990: f64, t1398: f64, t10079: f64, t3923: f64, t4089: f64, t40921: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46522, t46526, t46531, t46532, t46536, t46540, t46542) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1712(t3999, t4066, t1438, t40317, t4056, t543, t10065, t10069, t2782, t4086, t46469, t10084);
        let (t46547, t46551) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1713(t4003, t46477, t1437, t4004, t4114, t4118, t46518, t46520, t46522, t46526, t46532, t46536, t46540, t46542, t820, t9891, t9899);
        let (t46554, t46561, t46563, t46565, t46568, t46570) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1714(t1419, t9990, t1398, t2782, t4066, t4086, t543, t10069, t10079, t3923, t4089, t40921);
    (t46531, t46532, t46547, t46551, t46554, t46561, t46563, t46565, t46568, t46570)
}

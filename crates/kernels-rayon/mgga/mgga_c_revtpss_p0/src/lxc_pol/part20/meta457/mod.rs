//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta457 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1742;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1743;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1744;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1745;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta457(t1331: f64, t9342: f64, t9855: f64, t2619: f64, t9563: f64, t3825: f64, t9586: f64, t1333: f64, t14: f64, t27: f64, t521: f64, t583: f64, t596: f64, t39773: f64, t39783: f64, t39786: f64, t39791: f64, t39795: f64, t39799: f64, t30: f64, t525: f64, t9603: f64, t2257: f64, t3833: f64, t3834: f64, t39456: f64, t46311: f64, t46317: f64, t513: f64, t9335: f64, t9339: f64, t9344: f64, zeta_threshold: f64, t33: f64, t527: f64, t9615: f64, t3351: f64, t3841: f64, t3842: f64, t43744: f64, t46329: f64, t46335: f64, t516: f64, t9350: f64, t9354: f64, t9357: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47006, t47008, t47010, t47012, t47014, t47017, t47019) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1742(t1331, t9342, t9855, t2619, t9563, t3825, t9586, t1333, t14, t27, t521, t583, t596);
        let (t47020, t47021) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1743(t47019, t39773, t39783, t39786, t39791, t39795, t39799, t47006, t47008, t47010, t47012, t47014, t47017);
        let t47038 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1744(t30, t525, t9603, t2257, t3833, t3834, t39456, t46311, t46317, t513, t9335, t9339, t9344, zeta_threshold);
        let t47053 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1745(t33, t527, t9615, t3351, t3841, t3842, t43744, t46329, t46335, t516, t9350, t9354, t9357, zeta_threshold);
    (t47006, t47008, t47010, t47012, t47014, t47017, t47020, t47021, t47038, t47053)
}

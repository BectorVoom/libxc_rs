//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta457 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1742;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1743;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1744;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1745;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta457<F: Float>(t1331: F, t9342: F, t9855: F, t2619: F, t9563: F, t3825: F, t9586: F, t1333: F, t14: F, t27: F, t521: F, t583: F, t596: F, t39773: F, t39783: F, t39786: F, t39791: F, t39795: F, t39799: F, t30: F, t525: F, t9603: F, t2257: F, t3833: F, t3834: F, t39456: F, t46311: F, t46317: F, t513: F, t9335: F, t9339: F, t9344: F, zeta_threshold: F, t33: F, t527: F, t9615: F, t3351: F, t3841: F, t3842: F, t43744: F, t46329: F, t46335: F, t516: F, t9350: F, t9354: F, t9357: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t47006, t47008, t47010, t47012, t47014, t47017, t47019) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1742::<F>(t1331, t9342, t9855, t2619, t9563, t3825, t9586, t1333, t14, t27, t521, t583, t596);
        let (t47020, t47021) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1743::<F>(t47019, t39773, t39783, t39786, t39791, t39795, t39799, t47006, t47008, t47010, t47012, t47014, t47017);
        let t47038 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1744::<F>(t30, t525, t9603, t2257, t3833, t3834, t39456, t46311, t46317, t513, t9335, t9339, t9344, zeta_threshold);
        let t47053 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1745::<F>(t33, t527, t9615, t3351, t3841, t3842, t43744, t46329, t46335, t516, t9350, t9354, t9357, zeta_threshold);
    (t47006, t47008, t47010, t47012, t47014, t47017, t47020, t47021, t47038, t47053)
}

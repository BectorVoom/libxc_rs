//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta449 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1712;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1713;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1714;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta449<F: Float>(t3999: F, t4066: F, t1438: F, t40317: F, t4056: F, t543: F, t10065: F, t10069: F, t2782: F, t4086: F, t46469: F, t10084: F, t4003: F, t46477: F, t1437: F, t4004: F, t4114: F, t4118: F, t46518: F, t46520: F, t820: F, t9891: F, t9899: F, t1419: F, t9990: F, t1398: F, t10079: F, t3923: F, t4089: F, t40921: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t46522, t46526, t46531, t46532, t46536, t46540, t46542) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1712::<F>(t3999, t4066, t1438, t40317, t4056, t543, t10065, t10069, t2782, t4086, t46469, t10084);
        let (t46547, t46551) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1713::<F>(t4003, t46477, t1437, t4004, t4114, t4118, t46518, t46520, t46522, t46526, t46532, t46536, t46540, t46542, t820, t9891, t9899);
        let (t46554, t46561, t46563, t46565, t46568, t46570) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1714::<F>(t1419, t9990, t1398, t2782, t4066, t4086, t543, t10069, t10079, t3923, t4089, t40921);
    (t46531, t46532, t46547, t46551, t46554, t46561, t46563, t46565, t46568, t46570)
}

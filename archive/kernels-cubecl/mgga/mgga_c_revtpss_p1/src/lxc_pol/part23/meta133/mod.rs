//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta133 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk868;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk869;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk870;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk871;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk872;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk873;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk874;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk875;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta133<F: Float>(t3140: F, t3596: F, t460: F, t3303: F, t3603: F, t1243: F, t471: F, t498: F, t1330: F, t72: F, t757: F, t530: F, t566: F, t525: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3766 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk868::<F>(t3140, t3596);
        let t3767 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk869::<F>(t3766, t460);
        let t3769 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk870::<F>(t3303, t3603);
        let t3781 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk871::<F>(t1243, t3140);
        let t3782 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk872::<F>(t3781, t460);
        let t3783 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk873::<F>(t3303, t471);
        let (t3800, t3801) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk874::<F>(t498);
        let (t3825, t3826, t3828, t3833) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk875::<F>(t1330, t72, t757, t530, t566, t525);
    (t3766, t3767, t3769, t3781, t3782, t3783, t3800, t3801, t3825, t3826, t3828, t3833)
}

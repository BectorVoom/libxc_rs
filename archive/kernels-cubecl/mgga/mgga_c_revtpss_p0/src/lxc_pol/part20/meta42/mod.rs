//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta42 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk292;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk293;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk294;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk295;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk296;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk297;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk298;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk299;
use chunk8::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk300;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta42<F: Float>(t240: F, t843: F, t243: F, t247: F, t237: F, t233: F, t235: F, t239: F, t820: F, t205: F, t242: F, t72: F, t775: F, t828: F, t797: F, t799: F, t802: F, t812: F, t819: F, t825: F, t839: F, t225: F, t257: F, t213: F, t251: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t844 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk292::<F>(t240, t843);
        let (t848, t849) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk293::<F>(t243, t844, t247, t237, t233, t235);
        let t851 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk294::<F>(t239, t820, t849);
        let t853 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk295::<F>(t205, t242);
        let t854 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk296::<F>(t240, t853);
        let t855 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk297::<F>(t72, t854);
        let (t857, t860) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk298::<F>(t775, t828, t855, t797, t799, t802, t812, t819, t825, t839, t848, t851);
        let t861 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk299::<F>(t225, t860);
        let (t862, t865) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk300::<F>(t257, t861, t213, t251);
    (t844, t849, t851, t853, t854, t855, t857, t860, t861, t862, t865)
}

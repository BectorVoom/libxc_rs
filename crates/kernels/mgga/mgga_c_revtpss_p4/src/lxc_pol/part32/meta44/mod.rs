//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta44 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk282;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk283;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk284;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk285;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk286;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk287;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk288;
use chunk7::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk289;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta44<F: Float>(t243: F, t844: F, t247: F, t237: F, t233: F, t235: F, t239: F, t820: F, t205: F, t242: F, t240: F, t72: F, t775: F, t828: F, t797: F, t799: F, t802: F, t812: F, t819: F, t825: F, t839: F, t225: F, t257: F, t213: F, t251: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t846, t848, t849) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk282::<F>(t243, t844, t247, t237, t233, t235);
        let t851 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk283::<F>(t239, t820, t849);
        let t853 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk284::<F>(t205, t242);
        let t854 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk285::<F>(t240, t853);
        let t855 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk286::<F>(t72, t854);
        let t857 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk287::<F>(t775, t828, t855);
        let t860 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk288::<F>(t797, t799, t802, t812, t819, t825, t839, t848, t851, t857);
        let (t861, t862, t865) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk289::<F>(t225, t860, t257, t213, t251);
    (t846, t848, t849, t851, t853, t854, t855, t857, t860, t861, t862, t865)
}

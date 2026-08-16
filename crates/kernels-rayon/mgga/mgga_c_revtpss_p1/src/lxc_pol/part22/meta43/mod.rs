//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta43 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk320;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk321;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk322;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk323;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk324;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk325;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta43(t775: f64, t828: f64, t855: f64, t797: f64, t799: f64, t802: f64, t812: f64, t819: f64, t825: f64, t839: f64, t848: f64, t851: f64, t225: f64, t257: f64, t213: f64, t251: f64, t256: f64, t212: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t857, t860) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk320(t775, t828, t855, t797, t799, t802, t812, t819, t825, t839, t848, t851);
        let t861 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk321(t225, t860);
        let (t862, t865) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk322(t257, t861, t213, t251);
        let (t866, t867) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk323(t256);
        let t868 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk324(t225, t867);
        let t869 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk325(t212, t225);
    (t857, t860, t861, t862, t865, t866, t867, t868, t869)
}

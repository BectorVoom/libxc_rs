//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta238 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1067;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1068;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1069;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1070;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1071;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1072;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1073;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1074;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta238(t1699: f64, t1102: f64, t198: f64, t3336: f64, t336: f64, t6106: f64, t6108: f64, t6112: f64, t6144: f64, t6147: f64, t6213: f64, t6215: f64, t6217: f64, t6221: f64, t6225: f64, t6229: f64, t6396: f64, t30: f64, t265: f64, t393: f64, t6084: f64, t1468: f64, t1469: f64, t1587: f64, t1704: f64, t395: f64, t45: f64, t5824: f64, t5825: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t3362: f64, t5819: f64, t3360: f64, t128: f64, t3367: f64, t1120: f64, t1121: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6400, t6404) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1067(t1699, t1102, t198, t3336, t336, t6106, t6108, t6112, t6144, t6147, t6213, t6215, t6217, t6221, t6225, t6229, t6396);
        let (t6405, t6412) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1068(t30, t265, t393, t6084, t6404, t1468, t1469, t1587, t1704, t395, t45, t5824, t5825, dens_threshold, rho0, zeta_threshold);
        let t6416 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1069(t5824);
        let t6421 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1070(t3362, t5819);
        let (t6422, t6423) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1071(t3360, t6421, t128);
        let t6425 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1072(t3367, t5819);
        let (t6426, t6427) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1073(t1120, t6425, t128);
        let t6429 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1074(t1121, t5825);
    (t6400, t6405, t6412, t6416, t6421, t6422, t6423, t6425, t6426, t6427, t6429)
}

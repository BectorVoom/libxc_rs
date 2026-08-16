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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1067;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1068;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1069;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1070;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1071;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1072;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1073;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1074;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta238<F: Float>(t1699: F, t1102: F, t198: F, t3336: F, t336: F, t6106: F, t6108: F, t6112: F, t6144: F, t6147: F, t6213: F, t6215: F, t6217: F, t6221: F, t6225: F, t6229: F, t6396: F, t30: F, t265: F, t393: F, t6084: F, t1468: F, t1469: F, t1587: F, t1704: F, t395: F, t45: F, t5824: F, t5825: F, dens_threshold: F, rho0: F, zeta_threshold: F, t3362: F, t5819: F, t3360: F, t128: F, t3367: F, t1120: F, t1121: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t6400, t6404) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1067::<F>(t1699, t1102, t198, t3336, t336, t6106, t6108, t6112, t6144, t6147, t6213, t6215, t6217, t6221, t6225, t6229, t6396);
        let (t6405, t6412) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1068::<F>(t30, t265, t393, t6084, t6404, t1468, t1469, t1587, t1704, t395, t45, t5824, t5825, dens_threshold, rho0, zeta_threshold);
        let t6416 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1069::<F>(t5824);
        let t6421 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1070::<F>(t3362, t5819);
        let (t6422, t6423) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1071::<F>(t3360, t6421, t128);
        let t6425 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1072::<F>(t3367, t5819);
        let (t6426, t6427) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1073::<F>(t1120, t6425, t128);
        let t6429 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1074::<F>(t1121, t5825);
    (t6400, t6405, t6412, t6416, t6421, t6422, t6423, t6425, t6426, t6427, t6429)
}

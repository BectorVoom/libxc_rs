//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta662 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2242;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2243;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2244;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta662(t18245: f64, t7003: f64, t1518: f64, t4245: f64, t1937: f64, t1501: f64, t4292: f64, t30138: f64, t6993: f64, t29506: f64, t7316: f64, t30112: f64, t7235: f64, t27833: f64, t7937: f64, t28189: f64, t7898: f64, t7239: f64, t2014: f64, t30111: f64, t7315: f64, t109135: f64, t109138: f64, t109140: f64, t109142: f64, t109144: f64, t109147: f64, t2011: f64, t22506: f64, t5787: f64, t6934: f64, t7231: f64, t7894: f64, t28187: f64, t30110: f64, t531: f64, t7238: f64, t28043: f64, t7732: f64, t28021: f64, t28173: f64, t75439: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t109149, t109150, t109152, t109153, t109155, t109157, t109158, t109159) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2242(t18245, t7003, t1518, t4245, t1937, t1501, t4292, t30138, t6993, t29506, t7316, t30112, t7235);
        let t109170 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2243(t27833, t7937, t28189, t7898, t29506, t7239, t2014, t30111, t7315, t109135, t109138, t109140, t109142, t109144, t109147, t109149, t109152, t109155, t109157, t109158, t109159, t2011, t22506, t5787, t6934, t7231, t7894);
        let (t109172, t109176, t109178, t109180, t109182, t109194) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2244(t28187, t7898, t30110, t531, t2014, t7238, t28043, t7732, t28021, t28173, t1937, t75439);
    (t109150, t109153, t109170, t109172, t109176, t109178, t109180, t109182, t109194)
}

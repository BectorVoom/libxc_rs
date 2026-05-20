//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta662 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2242;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2243;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2244;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta662<F: Float>(t18245: F, t7003: F, t1518: F, t4245: F, t1937: F, t1501: F, t4292: F, t30138: F, t6993: F, t29506: F, t7316: F, t30112: F, t7235: F, t27833: F, t7937: F, t28189: F, t7898: F, t7239: F, t2014: F, t30111: F, t7315: F, t109135: F, t109138: F, t109140: F, t109142: F, t109144: F, t109147: F, t2011: F, t22506: F, t5787: F, t6934: F, t7231: F, t7894: F, t28187: F, t30110: F, t531: F, t7238: F, t28043: F, t7732: F, t28021: F, t28173: F, t75439: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t109149, t109150, t109152, t109153, t109155, t109157, t109158, t109159) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2242::<F>(t18245, t7003, t1518, t4245, t1937, t1501, t4292, t30138, t6993, t29506, t7316, t30112, t7235);
        let t109170 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2243::<F>(t27833, t7937, t28189, t7898, t29506, t7239, t2014, t30111, t7315, t109135, t109138, t109140, t109142, t109144, t109147, t109149, t109152, t109155, t109157, t109158, t109159, t2011, t22506, t5787, t6934, t7231, t7894);
        let (t109172, t109176, t109178, t109180, t109182, t109194) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2244::<F>(t28187, t7898, t30110, t531, t2014, t7238, t28043, t7732, t28021, t28173, t1937, t75439);
    (t109150, t109153, t109170, t109172, t109176, t109178, t109180, t109182, t109194)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2243/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2243(t27833: f64, t7937: f64, t28189: f64, t7898: f64, t29506: f64, t7239: f64, t2014: f64, t30111: f64, t7315: f64, t109135: f64, t109138: f64, t109140: f64, t109142: f64, t109144: f64, t109147: f64, t109149: f64, t109152: f64, t109155: f64, t109157: f64, t109158: f64, t109159: f64, t2011: f64, t22506: f64, t5787: f64, t6934: f64, t7231: f64, t7894: f64) -> f64 {
    let t109162 = 2.0_f64 * t27833 * t7937;
    let t109164 = 2.0_f64 * t7898 * t28189;
    let t109167 = 3.0_f64 * t29506 * t7239;
    let t109169 = t2014 * t30111 * t7315;
    let t109170 = t2011 * t22506 + 2.0_f64 * t5787 * t7894 + t6934 * t7231 + t109135 + t109138 + t109140 - t109142 - t109144 - t109147 - t109149 - t109152 - t109155 - t109157 - t109158 + t109159 - t109162 - t109164 + t109167 - t109169;
    t109170
}

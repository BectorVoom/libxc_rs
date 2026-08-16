//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta317 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1090;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1091;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1092;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta317(t123: f64, t1856: f64, t2630: f64, t1857: f64, t3860: f64, t5566: f64, t749: f64, t512: f64, t9856: f64, t1892: f64, t785: f64, t1358: f64, t2439: f64, t1903: f64, t4075: f64, t1444: f64, t556: f64, t2782: f64, t212: f64, t5710: f64, t689: f64, t221: f64, t3979: f64, t5591: f64, t3978: f64, t3989: f64, t5614: f64, t5622: f64, t9765: f64, t1408: f64, t240: f64, t1868: f64, t4010: f64, t1353: f64, t2661: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13666, t13668, t13682, t13683, t13726) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1090(t123, t1856, t2630, t1857, t3860, t5566, t749, t512, t9856, t1892, t785, t1358);
        let (t13727, t13733, t13737, t13760) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1091(t13726, t2439, t1903, t4075, t1444, t556, t2782, t212, t5710, t1358, t689, t221, t3979, t5591);
        let (t13762, t13763, t13765, t13772) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1092(t13760, t3978, t3989, t5614, t5622, t9765, t1408, t240, t1868, t4010, t1353, t2661);
    (t13666, t13668, t13682, t13683, t13727, t13733, t13737, t13762, t13763, t13765, t13772)
}

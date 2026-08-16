//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta317 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1090;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1091;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1092;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta317<F: Float>(t123: F, t1856: F, t2630: F, t1857: F, t3860: F, t5566: F, t749: F, t512: F, t9856: F, t1892: F, t785: F, t1358: F, t2439: F, t1903: F, t4075: F, t1444: F, t556: F, t2782: F, t212: F, t5710: F, t689: F, t221: F, t3979: F, t5591: F, t3978: F, t3989: F, t5614: F, t5622: F, t9765: F, t1408: F, t240: F, t1868: F, t4010: F, t1353: F, t2661: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13666, t13668, t13682, t13683, t13726) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1090::<F>(t123, t1856, t2630, t1857, t3860, t5566, t749, t512, t9856, t1892, t785, t1358);
        let (t13727, t13733, t13737, t13760) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1091::<F>(t13726, t2439, t1903, t4075, t1444, t556, t2782, t212, t5710, t1358, t689, t221, t3979, t5591);
        let (t13762, t13763, t13765, t13772) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1092::<F>(t13760, t3978, t3989, t5614, t5622, t9765, t1408, t240, t1868, t4010, t1353, t2661);
    (t13666, t13668, t13682, t13683, t13727, t13733, t13737, t13762, t13763, t13765, t13772)
}

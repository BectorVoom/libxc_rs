//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta345 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1355;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1356;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta345(t2496: f64, t5571: f64, t9597: f64, t123: f64, t1856: f64, t2630: f64, t1857: f64, t3860: f64, t5566: f64, t749: f64, t512: f64, t9856: f64, t1892: f64, t785: f64, t1358: f64, t2439: f64, t1903: f64, t4075: f64, t1444: f64, t556: f64, t2782: f64, t212: f64, t5710: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13652, t13664, t13666, t13668, t13682, t13683) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1355(t2496, t5571, t9597, t123, t1856, t2630, t1857, t3860, t5566, t749, t512, t9856);
        let (t13727, t13730, t13733, t13737) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1356(t1892, t785, t1358, t2439, t1903, t4075, t1444, t556, t2782, t212, t5710, t689);
    (t13652, t13664, t13666, t13668, t13682, t13683, t13727, t13730, t13733, t13737)
}

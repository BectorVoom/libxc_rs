//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta348 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1362;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1363;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta348<F: Float>(t2496: F, t5571: F, t9597: F, t123: F, t1856: F, t2630: F, t1857: F, t3860: F, t5566: F, t749: F, t512: F, t9856: F, t1892: F, t785: F, t1358: F, t2439: F, t1903: F, t4075: F, t1444: F, t556: F, t2782: F, t212: F, t5710: F, t689: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13652, t13664, t13666, t13668, t13682, t13683) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1362::<F>(t2496, t5571, t9597, t123, t1856, t2630, t1857, t3860, t5566, t749, t512, t9856);
        let (t13727, t13730, t13733, t13737) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1363::<F>(t1892, t785, t1358, t2439, t1903, t4075, t1444, t556, t2782, t212, t5710, t689);
    (t13652, t13664, t13666, t13668, t13682, t13683, t13727, t13730, t13733, t13737)
}

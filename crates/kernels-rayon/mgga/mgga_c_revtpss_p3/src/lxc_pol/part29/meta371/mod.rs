//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta371 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1330;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1331;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1332;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1333;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta371(t1892: f64, t785: f64, t1358: f64, t2439: f64, t1903: f64, t4075: f64, t1444: f64, t556: f64, t2782: f64, t212: f64, t5710: f64, t689: f64, t4131: f64, t4076: f64, t4077: f64, t9657: f64, t5774: f64, t10171: f64, t1424: f64, t1904: f64, t9632: f64, t9636: f64, t9639: f64, t9642: f64, t9650: f64, t13716: f64, t1414: f64, t828: f64, t221: f64, t3979: f64, t5591: f64, t3978: f64, t3989: f64, t5614: f64, t5622: f64, t9765: f64, t1408: f64, t240: f64, t1868: f64, t4010: f64, t1353: f64, t2661: f64, t1410: f64, t9697: f64, t9705: f64, t9711: f64, t9712: f64, t9716: f64, t9725: f64, t9729: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13727, t13730, t13733, t13737) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1330(t1892, t785, t1358, t2439, t1903, t4075, t1444, t556, t2782, t212, t5710, t689);
        let (t13739, t13743, t13747, t13750) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1331(t1903, t4131, t4076, t4077, t9657, t1444, t5774, t10171, t13727, t13733, t13737, t1424, t1904, t9632, t9636, t9639, t9642, t9650);
        let (t13756, t13760, t13762, t13763, t13765, t13767) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1332(t13716, t1414, t828, t221, t3979, t5591, t3978, t3989, t5614, t5622, t9765, t1408, t240);
        let (t13768, t13769, t13773) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1333(t1868, t4010, t1353, t13767, t2661, t13756, t13762, t13763, t13765, t1410, t9697, t9705, t9711, t9712, t9716, t9725, t9729);
    (t13730, t13739, t13743, t13747, t13750, t13756, t13760, t13768, t13769, t13773)
}

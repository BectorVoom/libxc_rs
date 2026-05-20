//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta371 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1330;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1331;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1332;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1333;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta371<F: Float>(t1892: F, t785: F, t1358: F, t2439: F, t1903: F, t4075: F, t1444: F, t556: F, t2782: F, t212: F, t5710: F, t689: F, t4131: F, t4076: F, t4077: F, t9657: F, t5774: F, t10171: F, t1424: F, t1904: F, t9632: F, t9636: F, t9639: F, t9642: F, t9650: F, t13716: F, t1414: F, t828: F, t221: F, t3979: F, t5591: F, t3978: F, t3989: F, t5614: F, t5622: F, t9765: F, t1408: F, t240: F, t1868: F, t4010: F, t1353: F, t2661: F, t1410: F, t9697: F, t9705: F, t9711: F, t9712: F, t9716: F, t9725: F, t9729: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13727, t13730, t13733, t13737) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1330::<F>(t1892, t785, t1358, t2439, t1903, t4075, t1444, t556, t2782, t212, t5710, t689);
        let (t13739, t13743, t13747, t13750) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1331::<F>(t1903, t4131, t4076, t4077, t9657, t1444, t5774, t10171, t13727, t13733, t13737, t1424, t1904, t9632, t9636, t9639, t9642, t9650);
        let (t13756, t13760, t13762, t13763, t13765, t13767) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1332::<F>(t13716, t1414, t828, t221, t3979, t5591, t3978, t3989, t5614, t5622, t9765, t1408, t240);
        let (t13768, t13769, t13773) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1333::<F>(t1868, t4010, t1353, t13767, t2661, t13756, t13762, t13763, t13765, t1410, t9697, t9705, t9711, t9712, t9716, t9725, t9729);
    (t13730, t13739, t13743, t13747, t13750, t13756, t13760, t13768, t13769, t13773)
}

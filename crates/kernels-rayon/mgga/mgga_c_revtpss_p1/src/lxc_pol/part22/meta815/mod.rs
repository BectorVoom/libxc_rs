//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta815 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2922;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2923;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2924;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta815(t1445: f64, t47567: f64, t10165: f64, t9664: f64, t1427: f64, t1444: f64, t22: f64, t9647: f64, t123: f64, t2434: f64, t4077: f64, t9680: f64, t125: f64, t1358: f64, t555: f64, t8779: f64, t9645: f64, t2435: f64, t9667: f64, t268: f64, t39644: f64, t556: f64, t561: f64, t786: f64, t9656: f64, t10150: f64, t2439: f64, t4066: f64, t785: f64, t9303: f64, t9641: f64, t9635: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47568, t47570, t47574, t47580) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2922(t1445, t47567, t10165, t9664, t1427, t1444, t22, t9647, t123, t2434, t4077, t9680);
        let (t47591, t47595, t47601) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2923(t123, t125, t1358, t555, t8779, t9645, t2435, t9667, t268, t39644, t556, t561);
        let (t47603, t47608, t47616, t47618, t47620) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2924(t556, t786, t9656, t10150, t2435, t1358, t2439, t4066, t785, t9303, t9641, t9635);
    (t47568, t47570, t47574, t47580, t47591, t47595, t47601, t47603, t47608, t47616, t47618, t47620)
}

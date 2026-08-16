//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta707 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2460;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2461;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta707(t10174: f64, t2453: f64, t1420: f64, t4075: f64, t786: f64, t1359: f64, t39501: f64, t10115: f64, t555: f64, t1445: f64, t10165: f64, t9664: f64, t1427: f64, t1444: f64, t22: f64, t9647: f64, t123: f64, t125: f64, t1358: f64, t8779: f64, t9645: f64, t268: f64, t39644: f64, t556: f64, t561: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47520, t47530, t47561, t47567, t47568, t47570) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2460(t10174, t2453, t1420, t4075, t786, t1359, t39501, t10115, t555, t1445, t10165, t9664);
        let (t47574, t47591, t47601) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2461(t1427, t1444, t22, t9647, t123, t125, t1358, t555, t8779, t9645, t268, t39644, t556, t561);
    (t47520, t47530, t47561, t47567, t47568, t47570, t47574, t47591, t47601)
}

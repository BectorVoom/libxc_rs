//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta465 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2146;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta465(t1079: f64, t15578: f64, t3215: f64, t4858: f64, t372: f64, t4872: f64, t4786: f64, t4873: f64, t11696: f64, t4781: f64, t3092: f64, t11705: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15579, t15583, t15584, t15585, t15586, t15591, t15592, t15595) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2146(t1079, t15578, t3215, t4858, t372, t4872, t4786, t4873, t11696, t4781, t3092, t11705);
    (t15579, t15583, t15584, t15585, t15586, t15591, t15592, t15595)
}

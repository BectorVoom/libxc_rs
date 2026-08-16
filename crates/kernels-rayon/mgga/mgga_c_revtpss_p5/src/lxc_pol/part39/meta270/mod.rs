//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1002;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1003;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1004;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta270(t555: f64, t9646: f64, t1358: f64, t22: f64, t1425: f64, t225: f64, t3907: f64, t9285: f64, t3906: f64, t1357: f64, t4132: f64, t689: f64, t4131: f64, t676: f64, t123: f64, t3915: f64, t2453: f64, t3914: f64, t1444: f64, t2438: f64, t138: f64, t4075: f64, t556: f64, t786: f64, t4077: f64, t2434: f64, t1359: f64, t9292: f64, t1363: f64, t9288: f64, t1362: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9650, t9657, t9666, t9668) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1002(t555, t9646, t1358, t22, t1425, t225, t3907, t9285, t3906, t1357, t4132, t689);
        let (t9672, t9674, t9677, t9680) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1003(t4131, t676, t123, t3915, t2453, t3914, t1444, t2438, t138, t4075, t556, t786);
        let (t9683, t9687, t9691, t9694) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1004(t4077, t676, t123, t9680, t1444, t2434, t3915, t1359, t9292, t1363, t9288, t1362);
    (t9650, t9657, t9666, t9668, t9672, t9674, t9677, t9680, t9683, t9687, t9691, t9694)
}

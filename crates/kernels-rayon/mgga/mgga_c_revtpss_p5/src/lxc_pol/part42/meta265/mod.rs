//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta265 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1008;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1009;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1010;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta265(t1333: f64, t3860: f64, t30: f64, t513: f64, t33: f64, t516: f64, t2435: f64, t3900: f64, t3896: f64, t9303: f64, t1419: f64, t785: f64, t1358: f64, t2439: f64, t784: f64, t209: f64, t555: f64, t22: f64, t1425: f64, t225: f64, t3907: f64, t9285: f64, t3906: f64, t2453: f64, t3914: f64, t1444: f64, t2438: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9597, t9605, t9617, t9632, t9639, t9640) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1008(t1333, t3860, t30, t513, t33, t516, t2435, t3900, t3896, t9303, t1419, t785);
        let (t9642, t9646) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1009(t1358, t9640, t2439, t784, t209);
        let (t9650, t9657, t9666, t9674, t9675) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1010(t555, t9646, t1358, t22, t1425, t225, t3907, t9285, t3906, t2453, t3914, t1444, t2438);
    (t9597, t9605, t9617, t9632, t9639, t9642, t9646, t9650, t9657, t9666, t9674, t9675)
}

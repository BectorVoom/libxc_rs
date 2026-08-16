//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta269 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1003;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1004;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta269(t1333: f64, t3860: f64, t4144: f64, t4147: f64, t30: f64, t513: f64, t33: f64, t516: f64, t2435: f64, t3900: f64, t212: f64, t4066: f64, t1358: f64, t689: f64, t3896: f64, t9303: f64, t1419: f64, t785: f64, t2439: f64, t784: f64, t209: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9597, t9599, t9605, t9617, t9632, t9634) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1003(t1333, t3860, t4144, t4147, t30, t513, t33, t516, t2435, t3900, t212, t4066);
        let (t9636, t9639, t9642, t9646) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1004(t1358, t9634, t689, t3896, t9303, t1419, t785, t2439, t784, t209);
    (t9597, t9599, t9605, t9617, t9632, t9636, t9639, t9642, t9646)
}

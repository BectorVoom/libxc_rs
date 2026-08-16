//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta629 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2074;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2075;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta629(t136: f64, t2457: f64, t7769: f64, t93377: f64, t4534: f64, t689: f64, t7014: f64, t27303: f64, t786: f64, t789: f64, t25296: f64, t27216: f64, t14991: f64, t93261: f64, t27213: f64, t92843: f64, t98815: f64, t27291: f64, t25431: f64, t25411: f64, t2453: f64, t27212: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t99211, t99212, t99216, t99219, t99222) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2074(t136, t2457, t7769, t93377, t4534, t689, t7014, t27303, t786, t789, t25296, t27216);
        let (t99228, t99231, t99234, t99243, t99245, t99257) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2075(t14991, t93261, t25296, t27213, t92843, t98815, t27291, t689, t25431, t25411, t2453, t27212);
    (t99211, t99212, t99216, t99219, t99222, t99228, t99231, t99234, t99243, t99245, t99257)
}

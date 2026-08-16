//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta359 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1172;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1173;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta359(t17350: f64, t3782: f64, t1263: f64, t1794: f64, t372: f64, t11262: f64, t1796: f64, t1247: f64, t12915: f64, t247: f64, t5230: f64, t5384: f64, t12772: f64, t5406: f64, t3625: f64, t1802: f64, t474: f64, t3089: f64, t3717: f64, t1284: f64, t5219: f64, t3624: f64, t1230: f64, t5390: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17351, t17353, t17362, t17375) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1172(t17350, t3782, t1263, t1794, t372, t11262, t1796, t1247, t12915, t247, t5230, t5384);
        let (t17386, t17394, t17395, t17396, t17401, t17412) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1173(t12772, t5406, t3625, t1802, t474, t3089, t3717, t1284, t5219, t3624, t1230, t5390);
    (t17351, t17353, t17362, t17375, t17386, t17394, t17395, t17396, t17401, t17412)
}

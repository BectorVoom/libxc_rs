//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta460 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1757;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1758;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta460(t2242: f64, t607: f64, t38: f64, t6972: f64, t2247: f64, t640: f64, t644: f64, t77: f64, t2315: f64, t84: f64, t2251: f64, t603: f64, t2259: f64, t48: f64, t613: f64, t2275: f64, t43: f64, t239: f64, t2258: f64, t2269: f64, t49: f64, t606: f64, t6968: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25102, t25105, t25106, t25110, t25114, t25117) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1757(t2242, t607, t38, t6972, t2247, t640, t644, t77, t2315, t84, t2251, t603);
        let (t25120, t25129, t25132, t25137, t25138) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1758(t2259, t603, t48, t613, t2275, t43, t239, t2251, t2258, t2269, t49, t606, t6968);
    (t25102, t25105, t25106, t25110, t25114, t25117, t25120, t25129, t25132, t25137, t25138)
}

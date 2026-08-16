//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta460 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1755;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1756;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta460(t2315: f64, t84: f64, t77: f64, t2251: f64, t603: f64, t2259: f64, t239: f64, t2311: f64, t76: f64, t10298: f64, t38: f64, t2248: f64, t2247: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25114, t25117, t25120, t25137, t25146, t25150, t25159) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1755(t2315, t84, t77, t2251, t603, t2259, t239, t2311, t76, t10298, t38, t2248);
        let t25162 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1756(t2247, t607);
    (t25114, t25117, t25120, t25137, t25146, t25150, t25159, t25162)
}

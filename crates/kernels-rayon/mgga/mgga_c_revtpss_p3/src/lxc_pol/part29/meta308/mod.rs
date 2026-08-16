//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta308 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1206;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1207;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta308(t15: f64, t588: f64, t11: f64, t2: f64, t22: f64, t2224: f64, t27: f64, t584: f64, t20: f64, t596: f64, t12: f64, t583: f64, t2231: f64, t2237: f64, t592: f64, t2236: f64, t3: f64, t25: f64, t2240: f64, t602: f64, t2246: f64, t599: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10275, t10278, t10279, t10281, t10284, t10285) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1206(t15, t588, t11, t2, t22, t2224, t27, t584, t20, t596, t12, t583);
        let (t10287, t10288, t10290, t10295, t10298, t10301) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1207(t10285, t27, t2231, t596, t2237, t592, t2236, t3, t25, t2240, t602, t2246, t599);
    (t10275, t10278, t10279, t10281, t10284, t10287, t10288, t10290, t10295, t10298, t10301)
}

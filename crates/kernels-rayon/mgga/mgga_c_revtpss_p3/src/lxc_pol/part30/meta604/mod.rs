//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta604 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2065;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2066;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta604(t12995: f64, t26824: f64, t12963: f64, t7613: f64, t12975: f64, t2138: f64, t12984: f64, t12851: f64, t2134: f64, t3567: f64, t8945: f64, t26894: f64, t29199: f64, t3596: f64, t37885: f64, t2149: f64, t1210: f64, t26936: f64, t3566: f64, t13181: f64, t3140: f64, t1243: f64, t2147: f64, t44841: f64, t7635: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97279, t97281, t97283, t97288, t97296, t97304, t97308) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2065(t12995, t26824, t12963, t7613, t12975, t2138, t12984, t12851, t2134, t3567, t8945, t26894, t29199);
        let (t97313, t97318, t97343, t97348, t97358) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2066(t3596, t37885, t2149, t1210, t29199, t26936, t3566, t13181, t3140, t1243, t2147, t44841, t7635);
    (t97279, t97281, t97283, t97288, t97296, t97304, t97308, t97313, t97318, t97343, t97348, t97358)
}

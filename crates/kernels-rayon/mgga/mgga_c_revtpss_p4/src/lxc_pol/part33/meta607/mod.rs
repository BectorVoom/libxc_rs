//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta607 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2032;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2033;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta607(t2149: f64, t97312: f64, t1294: f64, t5464: f64, t1210: f64, t29199: f64, t1203: f64, t21471: f64, t3596: f64, t7627: f64, t26936: f64, t3566: f64, t13181: f64, t3140: f64, t1243: f64, t2147: f64, t44841: f64, t7635: f64, t3572: f64, t8945: f64, t45551: f64, t473: f64, t37885: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97313, t97314, t97318, t97319, t97332, t97343) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2032(t2149, t97312, t1294, t5464, t1210, t29199, t1203, t21471, t3596, t7627, t26936, t3566);
        let (t97348, t97358, t97363, t97377, t97397) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2033(t13181, t3140, t1243, t2149, t2147, t44841, t7635, t3572, t8945, t45551, t473, t37885);
    (t97313, t97314, t97318, t97319, t97332, t97343, t97348, t97358, t97363, t97377, t97397)
}

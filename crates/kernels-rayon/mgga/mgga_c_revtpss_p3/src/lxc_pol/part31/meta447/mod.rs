//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta447 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1600;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1601;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1602;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1603;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta447(t20266: f64, t3360: f64, t128: f64, t3367: f64, t5825: f64, t606: f64, t1120: f64, t20283: f64, t20285: f64, t20287: f64, t20290: f64, t20295: f64, t20300: f64, t20304: f64, t20308: f64, t20312: f64, t12296: f64, t12297: f64, t16706: f64, t16915: f64, t16916: f64, t16917: f64, t1132: f64, t1145: f64, t141: f64, t20302: f64, t3417: f64, t20298: f64, t20310: f64, t20306: f64, t12327: f64, t6442: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t20315 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1600(t20266, t3360, t128);
        let (t20318, t20320) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1601(t3367, t5825, t606, t1120, t128);
        let (t20322, t20337) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1602(t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320, t12296, t12297, t16706, t16915, t16916, t16917);
        let (t20338, t20341, t20344, t20347, t20350, t20353, t20356) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1603(t1132, t20337, t1145, t20318, t141, t20302, t3417, t20298, t20310, t20306, t12327, t6442);
    (t20315, t20318, t20320, t20322, t20337, t20338, t20341, t20344, t20347, t20350, t20353, t20356)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta454 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2120;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2121;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2122;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2123;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta454(t15220: f64, t923: f64, t916: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11339: f64, t11366: f64, t11368: f64, t11479: f64, t11480: f64, t11326: f64, t15108: f64, t15111: f64, t15114: f64, t15116: f64, t15119: f64, t15121: f64, t15123: f64, t15125: f64, t15128: f64, t15132: f64, t15175: f64, t15178: f64, t15181: f64, t15184: f64, t15187: f64, t15189: f64, t15192: f64, t15195: f64, t15198: f64, t15200: f64, t973: f64, t2962: f64, t4673: f64, t11452: f64, t1621: f64, t2944: f64, t4708: f64, t972: f64, t1634: f64, t3006: f64, t2988: f64, t4711: f64, t3014: f64, t4707: f64, t11450: f64, t11461: f64, t11466: f64, t11554: f64, t15100: f64, t15103: f64, t15104: f64, t2945: f64, t2968: f64, t2987: f64, t3012: f64, t4690: f64, t4712: f64, t965: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15221, t15230, t15232) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2120(t15220, t923, t916, t11134, t11136, t11138, t11140, t11339, t11366, t11368, t11479, t11480);
        let t15234 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2121(t11326, t15108, t15111, t15114, t15116, t15119, t15121, t15123, t15125, t15128, t15132, t15175, t15178, t15181, t15184, t15187, t15189, t15192, t15195, t15198, t15200, t15232);
        let (t15235, t15238, t15242, t15249, t15252, t15255) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2122(t15234, t973, t2962, t4673, t11452, t1621, t2944, t4708, t972, t1634, t3006, t2988, t4711);
        let (t15258, t15259, t15262) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2123(t3014, t4707, t972, t11450, t11461, t11466, t11554, t15100, t15103, t15104, t15235, t15238, t15242, t15249, t15252, t15255, t2945, t2968, t2987, t3012, t4690, t4712, t965);
    (t15221, t15230, t15234, t15235, t15238, t15242, t15249, t15252, t15255, t15258, t15259, t15262)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta365 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1275;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1276;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1277;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1278;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta365(t15234: f64, t973: f64, t2962: f64, t4673: f64, t11452: f64, t1621: f64, t2944: f64, t4708: f64, t972: f64, t1634: f64, t3006: f64, t2988: f64, t4711: f64, t3014: f64, t4707: f64, t11450: f64, t11461: f64, t11466: f64, t11554: f64, t15100: f64, t15103: f64, t15104: f64, t2945: f64, t2968: f64, t2987: f64, t3012: f64, t4690: f64, t4712: f64, t965: f64, t11509: f64, t1633: f64, t4670: f64, t953: f64, t1622: f64, t2970: f64, t4669: f64, t15127: f64, t15168: f64, t15137: f64, t15142: f64, t15147: f64, t15151: f64, t15156: f64, t15160: f64, t15163: f64, t15166: f64, t15170: f64, t15173: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15235, t15238, t15242, t15249, t15252, t15255) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1275(t15234, t973, t2962, t4673, t11452, t1621, t2944, t4708, t972, t1634, t3006, t2988, t4711);
        let t15262 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1276(t3014, t4707, t972, t11450, t11461, t11466, t11554, t15100, t15103, t15104, t15235, t15238, t15242, t15249, t15252, t15255, t2945, t2968, t2987, t3012, t4690, t4712, t965);
        let (t15263, t15267, t15274, t15277, t15280, t15283) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1277(t3006, t4711, t11509, t1633, t2988, t4670, t953, t1622, t2962, t2944, t4673, t2970, t4669);
        let (t15284, t15287, t15290, t15301, t15315) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1278(t15283, t953, t1622, t2944, t1634, t2988, t15127, t15168, t15137, t15142, t15147, t15151, t15156, t15160, t15163, t15166, t15170, t15173);
    (t15262, t15263, t15267, t15274, t15277, t15280, t15284, t15287, t15290, t15301, t15315)
}

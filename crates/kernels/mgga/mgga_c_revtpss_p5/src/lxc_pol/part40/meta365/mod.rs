//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta365 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1278;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1279;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1280;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1281;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta365<F: Float>(t15234: F, t973: F, t2962: F, t4673: F, t11452: F, t1621: F, t2944: F, t4708: F, t972: F, t1634: F, t3006: F, t2988: F, t4711: F, t3014: F, t4707: F, t11450: F, t11461: F, t11466: F, t11554: F, t15100: F, t15103: F, t15104: F, t2945: F, t2968: F, t2987: F, t3012: F, t4690: F, t4712: F, t965: F, t11509: F, t1633: F, t4670: F, t953: F, t1622: F, t2970: F, t4669: F, t15127: F, t15168: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15163: F, t15166: F, t15170: F, t15173: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t15235, t15238, t15242, t15249, t15252, t15255) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1278::<F>(t15234, t973, t2962, t4673, t11452, t1621, t2944, t4708, t972, t1634, t3006, t2988, t4711);
        let t15262 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1279::<F>(t3014, t4707, t972, t11450, t11461, t11466, t11554, t15100, t15103, t15104, t15235, t15238, t15242, t15249, t15252, t15255, t2945, t2968, t2987, t3012, t4690, t4712, t965);
        let (t15263, t15267, t15274, t15277, t15280, t15283) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1280::<F>(t3006, t4711, t11509, t1633, t2988, t4670, t953, t1622, t2962, t2944, t4673, t2970, t4669);
        let (t15284, t15287, t15290, t15301, t15315) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1281::<F>(t15283, t953, t1622, t2944, t1634, t2988, t15127, t15168, t15137, t15142, t15147, t15151, t15156, t15160, t15163, t15166, t15170, t15173);
    (t15262, t15263, t15267, t15274, t15277, t15280, t15284, t15287, t15290, t15301, t15315)
}

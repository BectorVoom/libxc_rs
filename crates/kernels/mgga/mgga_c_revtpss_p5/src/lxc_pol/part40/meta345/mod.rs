//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta345 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1160;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1161;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1162;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1163;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta345<F: Float>(t13716: F, t1414: F, t828: F, t221: F, t3979: F, t5591: F, t3978: F, t3989: F, t5614: F, t5622: F, t9765: F, t1408: F, t240: F, t1868: F, t4010: F, t1353: F, t2661: F, t1410: F, t9697: F, t9705: F, t9711: F, t9712: F, t9716: F, t9725: F, t9729: F, t550: F, t5658: F, t543: F, t3992: F, t5610: F, t9775: F, t1889: F, t9779: F, t9954: F, t1398: F, t3938: F, t3935: F, t1882: F, t4003: F, t3957: F, t5690: F, t1873: F, t9741: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13756, t13762, t13763, t13765, t13767) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1160::<F>(t13716, t1414, t828, t221, t3979, t5591, t3978, t3989, t5614, t5622, t9765, t1408, t240);
        let (t13768, t13773) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1161::<F>(t1868, t4010, t1353, t13767, t2661, t13756, t13762, t13763, t13765, t1410, t9697, t9705, t9711, t9712, t9716, t9725, t9729);
        let (t13778, t13779, t13781, t13783, t13784) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1162::<F>(t550, t5658, t543, t3992, t2661, t5610, t9775, t1889, t9779, t828, t9954, t1398, t1868);
        let (t13786, t13789, t13790, t13793, t13797, t13798) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1163::<F>(t13784, t3938, t13783, t3935, t828, t1882, t4003, t1353, t1398, t3957, t5690, t1873, t9741);
    (t13768, t13773, t13778, t13779, t13781, t13786, t13789, t13790, t13793, t13797, t13798)
}

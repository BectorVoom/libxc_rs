//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta345 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1157;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1158;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1159;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1160;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta345(t13716: f64, t1414: f64, t828: f64, t221: f64, t3979: f64, t5591: f64, t3978: f64, t3989: f64, t5614: f64, t5622: f64, t9765: f64, t1408: f64, t240: f64, t1868: f64, t4010: f64, t1353: f64, t2661: f64, t1410: f64, t9697: f64, t9705: f64, t9711: f64, t9712: f64, t9716: f64, t9725: f64, t9729: f64, t550: f64, t5658: f64, t543: f64, t3992: f64, t5610: f64, t9775: f64, t1889: f64, t9779: f64, t9954: f64, t1398: f64, t3938: f64, t3935: f64, t1882: f64, t4003: f64, t3957: f64, t5690: f64, t1873: f64, t9741: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13756, t13762, t13763, t13765, t13767) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1157(t13716, t1414, t828, t221, t3979, t5591, t3978, t3989, t5614, t5622, t9765, t1408, t240);
        let (t13768, t13773) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1158(t1868, t4010, t1353, t13767, t2661, t13756, t13762, t13763, t13765, t1410, t9697, t9705, t9711, t9712, t9716, t9725, t9729);
        let (t13778, t13779, t13781, t13783, t13784) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1159(t550, t5658, t543, t3992, t2661, t5610, t9775, t1889, t9779, t828, t9954, t1398, t1868);
        let (t13786, t13789, t13790, t13793, t13797, t13798) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1160(t13784, t3938, t13783, t3935, t828, t1882, t4003, t1353, t1398, t3957, t5690, t1873, t9741);
    (t13768, t13773, t13778, t13779, t13781, t13786, t13789, t13790, t13793, t13797, t13798)
}

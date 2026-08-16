//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta460 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1669;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1670;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1671;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1672;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1673;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta460(t1248: f64, t6587: f64, t1250: f64, t3720: f64, t17183: f64, t5330: f64, t17737: f64, t5297: f64, t3626: f64, t1230: f64, t6594: f64, t1803: f64, t5261: f64, t12297: f64, t12678: f64, t16706: f64, t17319: f64, t17320: f64, t17321: f64, t20283: f64, t20285: f64, t20287: f64, t20290: f64, t20295: f64, t20300: f64, t20304: f64, t20308: f64, t20312: f64, t20315: f64, t20320: f64, t459: f64, t225: f64, t480: f64, t12832: f64, t17401: f64, t17736: f64, t17767: f64, t17771: f64, t17791: f64, t17792: f64, t3718: f64, t484: f64, t5335: f64, t5348: f64, t6690: f64, t20782: f64, t20828: f64, t20855: f64, t20910: f64, t20955: f64, t20993: f64, t21027: f64, t21057: f64, t21114: f64, t21146: f64, t21176: f64, t21196: f64, t21226: f64, t21264: f64, t21295: f64, t494: f64, t1294: f64, t6702: f64, t13182: f64, t1210: f64, t12628: f64, t1274: f64, t1295: f64, t1775: f64, t17973: f64, t17995: f64, t18005: f64, t18065: f64, t18097: f64, t1829: f64, t20741: f64, t20744: f64, t20748: f64, t20753: f64, t20756: f64, t20760: f64, t3572: f64, t460: f64, t5220: f64, t5225: f64, t5231: f64, t5246: f64, t5498: f64, t6588: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21298, t21300, t21306, t21310, t21313, t21316) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1669(t1248, t6587, t1250, t3720, t17183, t5330, t17737, t5297, t3626, t1230, t6594, t1803, t5261);
        let t21332 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1670(t12297, t12678, t16706, t17319, t17320, t17321, t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320);
        let (t21333, t21334, t21338) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1671(t21332, t459, t225, t480, t12832, t17401, t17736, t17767, t17771, t17791, t17792, t21300, t21306, t21310, t21313, t21316, t3718, t484, t5335, t5348, t6690);
        let t21342 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1672(t20782, t20828, t20855, t20910, t20955, t20993, t21027, t21057, t21114, t21146, t21176, t21196, t21226, t21264, t21295, t21338);
        let (t21348, t21357) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1673(t21342, t225, t494, t1294, t6702, t13182, t1210, t12628, t1274, t1295, t1775, t17973, t17995, t18005, t18065, t18097, t1829, t20741, t20744, t20748, t20753, t20756, t20760, t3572, t460, t5220, t5225, t5231, t5246, t5498, t6588);
    (t21298, t21300, t21310, t21332, t21333, t21334, t21342, t21348, t21357)
}

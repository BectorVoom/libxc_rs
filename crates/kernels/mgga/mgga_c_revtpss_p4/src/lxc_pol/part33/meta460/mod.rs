//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta460 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1669;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1670;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1671;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1672;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1673;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta460<F: Float>(t1248: F, t6587: F, t1250: F, t3720: F, t17183: F, t5330: F, t17737: F, t5297: F, t3626: F, t1230: F, t6594: F, t1803: F, t5261: F, t12297: F, t12678: F, t16706: F, t17319: F, t17320: F, t17321: F, t20283: F, t20285: F, t20287: F, t20290: F, t20295: F, t20300: F, t20304: F, t20308: F, t20312: F, t20315: F, t20320: F, t459: F, t225: F, t480: F, t12832: F, t17401: F, t17736: F, t17767: F, t17771: F, t17791: F, t17792: F, t3718: F, t484: F, t5335: F, t5348: F, t6690: F, t20782: F, t20828: F, t20855: F, t20910: F, t20955: F, t20993: F, t21027: F, t21057: F, t21114: F, t21146: F, t21176: F, t21196: F, t21226: F, t21264: F, t21295: F, t494: F, t1294: F, t6702: F, t13182: F, t1210: F, t12628: F, t1274: F, t1295: F, t1775: F, t17973: F, t17995: F, t18005: F, t18065: F, t18097: F, t1829: F, t20741: F, t20744: F, t20748: F, t20753: F, t20756: F, t20760: F, t3572: F, t460: F, t5220: F, t5225: F, t5231: F, t5246: F, t5498: F, t6588: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t21298, t21300, t21306, t21310, t21313, t21316) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1669::<F>(t1248, t6587, t1250, t3720, t17183, t5330, t17737, t5297, t3626, t1230, t6594, t1803, t5261);
        let t21332 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1670::<F>(t12297, t12678, t16706, t17319, t17320, t17321, t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320);
        let (t21333, t21334, t21338) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1671::<F>(t21332, t459, t225, t480, t12832, t17401, t17736, t17767, t17771, t17791, t17792, t21300, t21306, t21310, t21313, t21316, t3718, t484, t5335, t5348, t6690);
        let t21342 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1672::<F>(t20782, t20828, t20855, t20910, t20955, t20993, t21027, t21057, t21114, t21146, t21176, t21196, t21226, t21264, t21295, t21338);
        let (t21348, t21357) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1673::<F>(t21342, t225, t494, t1294, t6702, t13182, t1210, t12628, t1274, t1295, t1775, t17973, t17995, t18005, t18065, t18097, t1829, t20741, t20744, t20748, t20753, t20756, t20760, t3572, t460, t5220, t5225, t5231, t5246, t5498, t6588);
    (t21298, t21300, t21310, t21332, t21333, t21334, t21342, t21348, t21357)
}

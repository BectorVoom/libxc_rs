//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta457 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1658;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1659;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1660;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1661;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1662;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1663;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1664;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1665;
use chunk8::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1666;
use chunk9::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1667;
use chunk10::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1668;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta457<F: Float>(t20703: F, t247: F, t3719: F, t5357: F, t5373: F, t140: F, t6658: F, t1222: F, t6662: F, t1774: F, t5284: F, t1250: F, t3720: F, t1266: F, t17629: F, t21228: F, t21234: F, t21236: F, t21239: F, t21242: F, t3625: F, t3718: F, t5381: F, t5384: F, t5397: F, t20747: F, t369: F, t6593: F, t475: F, t467: F, t1260: F, t17307: F, t1256: F, t6602: F, t6595: F, t6598: F, t17344: F, t17396: F, t17401: F, t17721: F, t17763: F, t1808: F, t3647: F, t5270: F, t5348: F, t5354: F, t5386: F, t5391: F, t6683: F, t1248: F, t6587: F, t17183: F, t5330: F, t17737: F, t5297: F, t3626: F, t1230: F, t6594: F, t1803: F, t5261: F, t12297: F, t12678: F, t16706: F, t17319: F, t17320: F, t17321: F, t20283: F, t20285: F, t20287: F, t20290: F, t20295: F, t20300: F, t20304: F, t20308: F, t20312: F, t20315: F, t20320: F, t459: F, t225: F, t480: F, t12832: F, t17736: F, t17767: F, t17771: F, t17791: F, t17792: F, t484: F, t5335: F, t6690: F, t20782: F, t20828: F, t20855: F, t20910: F, t20955: F, t20993: F, t21027: F, t21057: F, t21114: F, t21146: F, t21176: F, t21196: F, t21226: F, t494: F, t1294: F, t6702: F, t13182: F, t1210: F, t12628: F, t1274: F, t1295: F, t1775: F, t17973: F, t17995: F, t18005: F, t18065: F, t18097: F, t1829: F, t20741: F, t20744: F, t20748: F, t20753: F, t20756: F, t20760: F, t3572: F, t460: F, t5220: F, t5225: F, t5231: F, t5246: F, t5498: F, t6588: F, t1828: F, t5245: F, t1277: F, t5497: F, t3736: F, t5428: F, t1204: F, t1770: F, t17986: F, t18054: F, t18062: F, t18087: F, t18114: F, t3556: F, t3561: F, t5251: F, t5414: F, t5423: F, t6580: F, t6697: F, t6703: F, t1811: F, t5219: F, t3737: F, t1269: F, t6628: F, t3783: F, t3769: F, t1280: F, t1287: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t21246, t21249, t21252, t21255, t21257, t21258) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1658::<F>(t20703, t247, t3719, t5357, t5373, t140, t6658, t1222, t6662, t1774, t5284, t1250);
        let t21264 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1659::<F>(t21258, t3720, t1222, t1266, t17629, t21228, t21234, t21236, t21239, t21242, t21246, t21249, t21252, t21255, t3625, t3718, t5381, t5384, t5397);
        let (t21267, t21272, t21275, t21283, t21285) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1660::<F>(t20747, t247, t3719, t369, t6593, t475, t467, t1260, t17307, t1256, t6602, t6595);
        let t21295 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1661::<F>(t1256, t6598, t1266, t17344, t17396, t17401, t17721, t17763, t1808, t21267, t21272, t21275, t21283, t21285, t3647, t5270, t5348, t5354, t5386, t5391, t6683);
        let (t21298, t21300, t21306, t21310, t21313, t21316) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1662::<F>(t1248, t6587, t1250, t3720, t17183, t5330, t17737, t5297, t3626, t1230, t6594, t1803, t5261);
        let t21332 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1663::<F>(t12297, t12678, t16706, t17319, t17320, t17321, t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320);
        let (t21333, t21338) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1664::<F>(t21332, t459, t225, t480, t12832, t17401, t17736, t17767, t17771, t17791, t17792, t21300, t21306, t21310, t21313, t21316, t3718, t484, t5335, t5348, t6690);
        let t21342 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1665::<F>(t20782, t20828, t20855, t20910, t20955, t20993, t21027, t21057, t21114, t21146, t21176, t21196, t21226, t21264, t21295, t21338);
        let t21357 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1666::<F>(t21342, t225, t494, t1294, t6702, t13182, t1210, t12628, t1274, t1295, t1775, t17973, t17995, t18005, t18065, t18097, t1829, t20741, t20744, t20748, t20753, t20756, t20760, t3572, t460, t5220, t5225, t5231, t5246, t5498, t6588);
        let t21393 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1667::<F>(t1828, t5245, t1277, t1774, t5497, t3736, t5428, t1204, t1210, t1770, t1775, t17986, t18054, t18062, t18087, t18114, t1829, t3556, t3561, t5220, t5246, t5251, t5414, t5423, t6580, t6588, t6697, t6703);
        let (t21394, t21408, t21416, t21427, t21430, t21436) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1668::<F>(t1811, t5219, t1828, t5497, t3737, t1269, t6628, t3783, t3769, t1280, t20703, t1287, t5284);
    (t21257, t21298, t21333, t21342, t21357, t21393, t21394, t21408, t21416, t21427, t21430, t21436)
}

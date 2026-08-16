//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta435 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1562;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1563;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1564;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1565;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1566;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1567;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1568;
use chunk7::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1569;
use chunk8::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1570;
use chunk9::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1571;
use chunk10::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1572;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta435(t20703: f64, t247: f64, t3719: f64, t5357: f64, t5373: f64, t140: f64, t6658: f64, t1222: f64, t6662: f64, t1774: f64, t5284: f64, t1250: f64, t3720: f64, t1266: f64, t17629: f64, t21228: f64, t21234: f64, t21236: f64, t21239: f64, t21242: f64, t3625: f64, t3718: f64, t5381: f64, t5384: f64, t5397: f64, t20747: f64, t369: f64, t6593: f64, t475: f64, t467: f64, t1260: f64, t17307: f64, t1256: f64, t6602: f64, t6595: f64, t6598: f64, t17344: f64, t17396: f64, t17401: f64, t17721: f64, t17763: f64, t1808: f64, t3647: f64, t5270: f64, t5348: f64, t5354: f64, t5386: f64, t5391: f64, t6683: f64, t1248: f64, t6587: f64, t17183: f64, t5330: f64, t17737: f64, t5297: f64, t3626: f64, t1230: f64, t6594: f64, t1803: f64, t5261: f64, t12297: f64, t12678: f64, t16706: f64, t17319: f64, t17320: f64, t17321: f64, t20283: f64, t20285: f64, t20287: f64, t20290: f64, t20295: f64, t20300: f64, t20304: f64, t20308: f64, t20312: f64, t20315: f64, t20320: f64, t459: f64, t225: f64, t480: f64, t12832: f64, t17736: f64, t17767: f64, t17771: f64, t17791: f64, t17792: f64, t484: f64, t5335: f64, t6690: f64, t20782: f64, t20828: f64, t20855: f64, t20910: f64, t20955: f64, t20993: f64, t21027: f64, t21057: f64, t21114: f64, t21146: f64, t21176: f64, t21196: f64, t21226: f64, t494: f64, t1294: f64, t6702: f64, t13182: f64, t1210: f64, t12628: f64, t1274: f64, t1295: f64, t1775: f64, t17973: f64, t17995: f64, t18005: f64, t18065: f64, t18097: f64, t1829: f64, t20741: f64, t20744: f64, t20748: f64, t20753: f64, t20756: f64, t20760: f64, t3572: f64, t460: f64, t5220: f64, t5225: f64, t5231: f64, t5246: f64, t5498: f64, t6588: f64, t1828: f64, t5245: f64, t1277: f64, t5497: f64, t3736: f64, t5428: f64, t1204: f64, t1770: f64, t17986: f64, t18054: f64, t18062: f64, t18087: f64, t18114: f64, t3556: f64, t3561: f64, t5251: f64, t5414: f64, t5423: f64, t6580: f64, t6697: f64, t6703: f64, t1811: f64, t5219: f64, t3737: f64, t1269: f64, t6628: f64, t3783: f64, t3769: f64, t1280: f64, t1287: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21246, t21249, t21252, t21255, t21257, t21258) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1562(t20703, t247, t3719, t5357, t5373, t140, t6658, t1222, t6662, t1774, t5284, t1250);
        let t21264 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1563(t21258, t3720, t1222, t1266, t17629, t21228, t21234, t21236, t21239, t21242, t21246, t21249, t21252, t21255, t3625, t3718, t5381, t5384, t5397);
        let (t21267, t21272, t21275, t21283, t21285) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1564(t20747, t247, t3719, t369, t6593, t475, t467, t1260, t17307, t1256, t6602, t6595);
        let t21295 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1565(t1256, t6598, t1266, t17344, t17396, t17401, t17721, t17763, t1808, t21267, t21272, t21275, t21283, t21285, t3647, t5270, t5348, t5354, t5386, t5391, t6683);
        let (t21298, t21300, t21306, t21310, t21313, t21316) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1566(t1248, t6587, t1250, t3720, t17183, t5330, t17737, t5297, t3626, t1230, t6594, t1803, t5261);
        let t21332 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1567(t12297, t12678, t16706, t17319, t17320, t17321, t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320);
        let (t21333, t21338) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1568(t21332, t459, t225, t480, t12832, t17401, t17736, t17767, t17771, t17791, t17792, t21300, t21306, t21310, t21313, t21316, t3718, t484, t5335, t5348, t6690);
        let t21342 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1569(t20782, t20828, t20855, t20910, t20955, t20993, t21027, t21057, t21114, t21146, t21176, t21196, t21226, t21264, t21295, t21338);
        let t21357 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1570(t21342, t225, t494, t1294, t6702, t13182, t1210, t12628, t1274, t1295, t1775, t17973, t17995, t18005, t18065, t18097, t1829, t20741, t20744, t20748, t20753, t20756, t20760, t3572, t460, t5220, t5225, t5231, t5246, t5498, t6588);
        let t21393 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1571(t1828, t5245, t1277, t1774, t5497, t3736, t5428, t1204, t1210, t1770, t1775, t17986, t18054, t18062, t18087, t18114, t1829, t3556, t3561, t5220, t5246, t5251, t5414, t5423, t6580, t6588, t6697, t6703);
        let (t21394, t21408, t21416, t21427, t21430, t21436) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1572(t1811, t5219, t1828, t5497, t3737, t1269, t6628, t3783, t3769, t1280, t20703, t1287, t5284);
    (t21257, t21298, t21333, t21342, t21357, t21393, t21394, t21408, t21416, t21427, t21430, t21436)
}

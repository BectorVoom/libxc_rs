//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1045 (260520-c91 hierarchical CSE).
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
mod chunk11;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3659;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3660;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3661;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3662;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3663;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3664;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3665;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3666;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3667;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3668;
use chunk10::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3669;
use chunk11::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3670;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1045(t68297: f64, t68301: f64, t68305: f64, t68310: f64, t68332: f64, t68334: f64, t68336: f64, t68342: f64, t68347: f64, t68350: f64, t68353: f64, t68357: f64, t68360: f64, t56176: f64, t56183: f64, t56185: f64, t56187: f64, t56189: f64, t56209: f64, t56212: f64, t56214: f64, t56216: f64, t56228: f64, t68363: f64, t68366: f64, t43865: f64, t43888: f64, t43890: f64, t43892: f64, t56230: f64, t56236: f64, t68389: f64, t68393: f64, t68397: f64, t68399: f64, t68454: f64, t68456: f64, t68459: f64, t1168: f64, t1187: f64, t12476: f64, t16948: f64, t16959: f64, t16979: f64, t16989: f64, t435: f64, t57944: f64, t57972: f64, t58237: f64, t58259: f64, t58300: f64, t58345: f64, t58592: f64, t58647: f64, t6535: f64, t68250: f64, t68633: f64, t68636: f64, t68640: f64, t68694: f64, t68711: f64, t68730: f64, t68751: f64, t68754: f64, t68757: f64, t69139: f64, t12429: f64, t12486: f64, t12553: f64, t16971: f64, t17097: f64, t17151: f64, t20678: f64, t20679: f64, t3453: f64, t3477: f64, t3497: f64, t3515: f64, t3521: f64, t45061: f64, t45174: f64, t5158: f64, t6487: f64, t6503: f64, t6519: f64, t68760: f64, t68763: f64, t68766: f64, t68769: f64, t68772: f64, t68779: f64, t68781: f64, t68784: f64, t68791: f64, t68794: f64, t43771: f64, t43781: f64, t43783: f64, t45106: f64, t45107: f64, t68253: f64, t68255: f64, t68257: f64, t68262: f64, t68267: f64, t68271: f64, t68275: f64, t68277: f64, t68282: f64, t68287: f64, t68292: f64, t68312: f64, t68315: f64, t68319: f64, t68322: f64, t68326: f64, t68330: f64, t43911: f64, t68368: f64, t68370: f64, t68373: f64, t68402: f64, t68464: f64, t58145: f64, t58147: f64, t68470: f64, t68473: f64, t68476: f64, t68479: f64, t68481: f64, t68484: f64, t68486: f64, t68488: f64, t68490: f64, t68493: f64, t68495: f64, t68497: f64, t58153: f64, t58158: f64, t58160: f64, t58162: f64, t58165: f64, t58186: f64, t68507: f64, t68515: f64, t68518: f64, t68521: f64, t68524: f64, t58207: f64, t68529: f64, t68532: f64, t68535: f64, t68538: f64, t68540: f64, t68543: f64, t68546: f64, t68548: f64, t68550: f64, t68553: f64, t68556: f64, t68559: f64, t68561: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t69153 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3659(t68297, t68301, t68305, t68310, t68332, t68334, t68336, t68342, t68347, t68350, t68353, t68357, t68360);
        let t69167 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3660(t56176, t56183, t56185, t56187, t56189, t56209, t56212, t56214, t56216, t56228, t68363, t68366);
        let t69181 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3661(t43865, t43888, t43890, t43892, t56230, t56236, t68389, t68393, t68397, t68399, t68454, t68456, t68459);
        let t69192 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3662(t1168, t1187, t12476, t16948, t16959, t16979, t16989, t435, t57944, t57972, t58237, t58259, t58300, t58345, t58592, t58647, t6535, t68250, t68633, t68636, t68640, t68694, t68711, t68730, t68751, t68754, t68757, t69139, t69153, t69167, t69181);
        let t69216 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3663(t12429, t12486, t12553, t16971, t17097, t17151, t20678, t20679, t3453, t3477, t3497, t3515, t3521, t45061, t45174, t5158, t6487, t6503, t6519, t6535, t68760, t68763, t68766, t68769, t68772, t68779, t68781, t68784, t68791, t68794);
        let t69230 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3664(t43771, t43781, t43783, t45106, t45107, t68253, t68255, t68257, t68262, t68267, t68271, t68275, t68277, t68282);
        let t69246 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3665(t68287, t68292, t68297, t68301, t68305, t68310, t68312, t68315, t68319, t68322, t68326, t68330, t68332, t68334, t68336);
        let t69263 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3666(t43911, t56176, t56183, t56185, t68342, t68347, t68350, t68353, t68357, t68360, t68363, t68366, t68368, t68370, t68373);
        let t69279 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3667(t56187, t56189, t56209, t56212, t56214, t56216, t56228, t56230, t56236, t68389, t68393, t68397, t68399, t68402, t68464);
        let t69296 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3668(t58145, t58147, t68470, t68473, t68476, t68479, t68481, t68484, t68486, t68488, t68490, t68493, t68495, t68497);
        let t69312 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3669(t43865, t43888, t43890, t43892, t58153, t58158, t58160, t58162, t58165, t58186, t68507, t68515, t68518, t68521, t68524);
        let t69329 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3670(t58207, t68454, t68529, t68532, t68535, t68538, t68540, t68543, t68546, t68548, t68550, t68553, t68556, t68559, t68561);
    (t69192, t69216, t69230, t69246, t69263, t69279, t69296, t69312, t69329)
}

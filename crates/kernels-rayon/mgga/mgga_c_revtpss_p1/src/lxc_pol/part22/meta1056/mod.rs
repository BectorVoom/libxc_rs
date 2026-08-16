//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1056 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3737;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3738;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3739;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3740;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3741;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3742;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3743;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3744;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3745;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3746;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1056(t12772: f64, t17736: f64, t21309: f64, t3767: f64, t70629: f64, t474: f64, t6593: f64, t3089: f64, t1285: f64, t17384: f64, t17605: f64, t1012: f64, t1222: f64, t12256: f64, t15936: f64, t17658: f64, t17729: f64, t1774: f64, t21030: f64, t21121: f64, t3631: f64, t3692: f64, t3720: f64, t44225: f64, t44484: f64, t44609: f64, t44664: f64, t44675: f64, t44696: f64, t57786: f64, t58777: f64, t58831: f64, t60717: f64, t6688: f64, t17448: f64, t17451: f64, t1121: f64, t6587: f64, t13148: f64, t70916: f64, t13142: f64, t21218: f64, t3625: f64, t12784: f64, t12787: f64, t12855: f64, t12910: f64, t17429: f64, t17459: f64, t17713: f64, t17730: f64, t17750: f64, t20297: f64, t20838: f64, t21008: f64, t21119: f64, t21164: f64, t21257: f64, t3626: f64, t5354: f64, t5407: f64, t57040: f64, t57571: f64, t58791: f64, t1250: f64, t5245: f64, t1794: f64, t372: f64, t5277: f64, t17395: f64, t17400: f64, t12702: f64, t12744: f64, t16725: f64, t17354: f64, t17657: f64, t17688: f64, t17693: f64, t20795: f64, t20945: f64, t20946: f64, t20947: f64, t21013: f64, t3617: f64, t3723: f64, t44510: f64, t44517: f64, t5284: f64, t5331: f64, t5335: f64, t5343: f64, t5346: f64, t56888: f64, t58824: f64, t58827: f64, t20809: f64, t21172: f64, t44307: f64, t68253: f64, t68255: f64, t68257: f64, t68262: f64, t68267: f64, t68271: f64, t68275: f64, t68277: f64, t68282: f64, t68287: f64, t68292: f64, t68297: f64, t68301: f64, t68305: f64, t68310: f64, t68332: f64, t68334: f64, t68336: f64, t68342: f64, t68347: f64, t68350: f64, t68353: f64, t68357: f64, t68360: f64, t56176: f64, t56183: f64, t56185: f64, t56187: f64, t56189: f64, t56209: f64, t56212: f64, t56214: f64, t56216: f64, t56228: f64, t68363: f64, t68366: f64, t43865: f64, t43888: f64, t43890: f64, t43892: f64, t56230: f64, t56236: f64, t68389: f64, t68393: f64, t68397: f64, t68399: f64, t68454: f64, t68456: f64, t68459: f64, t459: f64, t3655: f64, t6598: f64, t6602: f64, t12705: f64, t12712: f64, t12866: f64, t17351: f64, t17353: f64, t17638: f64, t21020: f64, t21040: f64, t225: f64, t3630: f64, t44585: f64, t44704: f64, t480: f64, t484: f64, t56879: f64, t57548: f64, t57550: f64, t57606: f64, t58850: f64, t58853: f64, t60927: f64, t6638: f64, t1715: f64, t3601: f64, t20816: f64, t3708: f64, t13053: f64, t17475: f64, t17640: f64, t17650: f64, t44521: f64, t44751: f64, t5330: f64, t57480: f64, t58868: f64, t58878: f64, t58882: f64, t58884: f64, t59066: f64, t59854: f64, t68265: f64, t68308: f64, t68345: f64, t17183: f64, t17350: f64, t20944: f64, t3153: f64, t12809: f64, t17355: f64, t17464: f64, t17646: f64, t17654: f64, t17694: f64, t21004: f64, t3604: f64, t3611: f64, t44190: f64, t44624: f64, t5308: f64, t5312: f64, t5373: f64, t58889: f64, t58897: f64, t68280: f64, t68295: f64, t69844: f64, t70933: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t70994, t71015) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3737(t12772, t17736, t21309, t3767, t70629, t474, t6593, t3089, t1285, t17384, t17605, t1012, t1222, t12256, t15936, t17658, t17729, t1774, t21030, t21121, t3631, t3692, t3720, t44225, t44484, t44609, t44664, t44675, t44696, t57786, t58777, t58831, t60717, t6688);
        let t71053 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3738(t17448, t17451, t1121, t6587, t13148, t70916, t13142, t12772, t21218, t3625, t12784, t12787, t12855, t12910, t17429, t17459, t17713, t17729, t17730, t17736, t17750, t20297, t20838, t21008, t21119, t21164, t21257, t3626, t3720, t5354, t5407, t57040, t57571, t58791);
        let (t71061, t71098) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3739(t1250, t5245, t1794, t372, t5277, t17395, t17400, t12702, t12744, t12787, t12855, t12910, t16725, t17354, t17657, t17688, t17693, t20795, t20945, t20946, t20947, t21013, t21119, t21164, t3617, t3720, t3723, t44510, t44517, t5284, t5331, t5335, t5343, t5346, t56888, t58824, t58827);
        let (t71112, t71117, t71134) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3740(t20809, t372, t12772, t21172, t5331, t44307, t68253, t68255, t68257, t68262, t68267, t68271, t68275, t68277, t68282, t68287, t68292);
        let t71148 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3741(t68297, t68301, t68305, t68310, t68332, t68334, t68336, t68342, t68347, t68350, t68353, t68357, t68360);
        let t71162 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3742(t56176, t56183, t56185, t56187, t56189, t56209, t56212, t56214, t56216, t56228, t68363, t68366);
        let t71176 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3743(t43865, t43888, t43890, t43892, t56230, t56236, t68389, t68393, t68397, t68399, t68454, t68456, t68459);
        let (t71179, t71196) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3744(t459, t71134, t71148, t71162, t71176, t3655, t6598, t6602, t12705, t12712, t12866, t17351, t17353, t17638, t21020, t21040, t225, t3625, t3626, t3630, t44585, t44704, t480, t484, t56879, t57548, t57550, t57606, t58850, t58853, t60927, t6638, t71112, t71117);
        let (t71200, t71231) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3745(t1715, t3601, t20816, t3708, t1121, t1222, t13053, t17353, t17448, t17475, t17640, t17650, t372, t44521, t44751, t5277, t5330, t5335, t57480, t58868, t58878, t58882, t58884, t59066, t59854, t68265, t68308, t68345);
        let (t71245, t71258, t71269) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3746(t17384, t17448, t17183, t17350, t20944, t3153, t372, t3601, t6587, t1222, t12809, t12855, t17355, t17464, t17646, t17654, t17693, t17694, t21004, t3604, t3611, t3720, t44190, t44624, t5308, t5312, t5373, t58889, t58897, t68280, t68295, t69844, t70933);
    (t70994, t71015, t71053, t71061, t71098, t71179, t71196, t71200, t71231, t71245, t71258, t71269)
}

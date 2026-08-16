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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1056<F: Float>(t12772: F, t17736: F, t21309: F, t3767: F, t70629: F, t474: F, t6593: F, t3089: F, t1285: F, t17384: F, t17605: F, t1012: F, t1222: F, t12256: F, t15936: F, t17658: F, t17729: F, t1774: F, t21030: F, t21121: F, t3631: F, t3692: F, t3720: F, t44225: F, t44484: F, t44609: F, t44664: F, t44675: F, t44696: F, t57786: F, t58777: F, t58831: F, t60717: F, t6688: F, t17448: F, t17451: F, t1121: F, t6587: F, t13148: F, t70916: F, t13142: F, t21218: F, t3625: F, t12784: F, t12787: F, t12855: F, t12910: F, t17429: F, t17459: F, t17713: F, t17730: F, t17750: F, t20297: F, t20838: F, t21008: F, t21119: F, t21164: F, t21257: F, t3626: F, t5354: F, t5407: F, t57040: F, t57571: F, t58791: F, t1250: F, t5245: F, t1794: F, t372: F, t5277: F, t17395: F, t17400: F, t12702: F, t12744: F, t16725: F, t17354: F, t17657: F, t17688: F, t17693: F, t20795: F, t20945: F, t20946: F, t20947: F, t21013: F, t3617: F, t3723: F, t44510: F, t44517: F, t5284: F, t5331: F, t5335: F, t5343: F, t5346: F, t56888: F, t58824: F, t58827: F, t20809: F, t21172: F, t44307: F, t68253: F, t68255: F, t68257: F, t68262: F, t68267: F, t68271: F, t68275: F, t68277: F, t68282: F, t68287: F, t68292: F, t68297: F, t68301: F, t68305: F, t68310: F, t68332: F, t68334: F, t68336: F, t68342: F, t68347: F, t68350: F, t68353: F, t68357: F, t68360: F, t56176: F, t56183: F, t56185: F, t56187: F, t56189: F, t56209: F, t56212: F, t56214: F, t56216: F, t56228: F, t68363: F, t68366: F, t43865: F, t43888: F, t43890: F, t43892: F, t56230: F, t56236: F, t68389: F, t68393: F, t68397: F, t68399: F, t68454: F, t68456: F, t68459: F, t459: F, t3655: F, t6598: F, t6602: F, t12705: F, t12712: F, t12866: F, t17351: F, t17353: F, t17638: F, t21020: F, t21040: F, t225: F, t3630: F, t44585: F, t44704: F, t480: F, t484: F, t56879: F, t57548: F, t57550: F, t57606: F, t58850: F, t58853: F, t60927: F, t6638: F, t1715: F, t3601: F, t20816: F, t3708: F, t13053: F, t17475: F, t17640: F, t17650: F, t44521: F, t44751: F, t5330: F, t57480: F, t58868: F, t58878: F, t58882: F, t58884: F, t59066: F, t59854: F, t68265: F, t68308: F, t68345: F, t17183: F, t17350: F, t20944: F, t3153: F, t12809: F, t17355: F, t17464: F, t17646: F, t17654: F, t17694: F, t21004: F, t3604: F, t3611: F, t44190: F, t44624: F, t5308: F, t5312: F, t5373: F, t58889: F, t58897: F, t68280: F, t68295: F, t69844: F, t70933: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t70994, t71015) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3737::<F>(t12772, t17736, t21309, t3767, t70629, t474, t6593, t3089, t1285, t17384, t17605, t1012, t1222, t12256, t15936, t17658, t17729, t1774, t21030, t21121, t3631, t3692, t3720, t44225, t44484, t44609, t44664, t44675, t44696, t57786, t58777, t58831, t60717, t6688);
        let t71053 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3738::<F>(t17448, t17451, t1121, t6587, t13148, t70916, t13142, t12772, t21218, t3625, t12784, t12787, t12855, t12910, t17429, t17459, t17713, t17729, t17730, t17736, t17750, t20297, t20838, t21008, t21119, t21164, t21257, t3626, t3720, t5354, t5407, t57040, t57571, t58791);
        let (t71061, t71098) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3739::<F>(t1250, t5245, t1794, t372, t5277, t17395, t17400, t12702, t12744, t12787, t12855, t12910, t16725, t17354, t17657, t17688, t17693, t20795, t20945, t20946, t20947, t21013, t21119, t21164, t3617, t3720, t3723, t44510, t44517, t5284, t5331, t5335, t5343, t5346, t56888, t58824, t58827);
        let (t71112, t71117, t71134) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3740::<F>(t20809, t372, t12772, t21172, t5331, t44307, t68253, t68255, t68257, t68262, t68267, t68271, t68275, t68277, t68282, t68287, t68292);
        let t71148 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3741::<F>(t68297, t68301, t68305, t68310, t68332, t68334, t68336, t68342, t68347, t68350, t68353, t68357, t68360);
        let t71162 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3742::<F>(t56176, t56183, t56185, t56187, t56189, t56209, t56212, t56214, t56216, t56228, t68363, t68366);
        let t71176 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3743::<F>(t43865, t43888, t43890, t43892, t56230, t56236, t68389, t68393, t68397, t68399, t68454, t68456, t68459);
        let (t71179, t71196) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3744::<F>(t459, t71134, t71148, t71162, t71176, t3655, t6598, t6602, t12705, t12712, t12866, t17351, t17353, t17638, t21020, t21040, t225, t3625, t3626, t3630, t44585, t44704, t480, t484, t56879, t57548, t57550, t57606, t58850, t58853, t60927, t6638, t71112, t71117);
        let (t71200, t71231) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3745::<F>(t1715, t3601, t20816, t3708, t1121, t1222, t13053, t17353, t17448, t17475, t17640, t17650, t372, t44521, t44751, t5277, t5330, t5335, t57480, t58868, t58878, t58882, t58884, t59066, t59854, t68265, t68308, t68345);
        let (t71245, t71258, t71269) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3746::<F>(t17384, t17448, t17183, t17350, t20944, t3153, t372, t3601, t6587, t1222, t12809, t12855, t17355, t17464, t17646, t17654, t17693, t17694, t21004, t3604, t3611, t3720, t44190, t44624, t5308, t5312, t5373, t58889, t58897, t68280, t68295, t69844, t70933);
    (t70994, t71015, t71053, t71061, t71098, t71179, t71196, t71200, t71231, t71245, t71258, t71269)
}

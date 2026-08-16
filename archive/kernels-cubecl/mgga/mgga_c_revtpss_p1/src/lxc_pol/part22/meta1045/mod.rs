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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1045<F: Float>(t68297: F, t68301: F, t68305: F, t68310: F, t68332: F, t68334: F, t68336: F, t68342: F, t68347: F, t68350: F, t68353: F, t68357: F, t68360: F, t56176: F, t56183: F, t56185: F, t56187: F, t56189: F, t56209: F, t56212: F, t56214: F, t56216: F, t56228: F, t68363: F, t68366: F, t43865: F, t43888: F, t43890: F, t43892: F, t56230: F, t56236: F, t68389: F, t68393: F, t68397: F, t68399: F, t68454: F, t68456: F, t68459: F, t1168: F, t1187: F, t12476: F, t16948: F, t16959: F, t16979: F, t16989: F, t435: F, t57944: F, t57972: F, t58237: F, t58259: F, t58300: F, t58345: F, t58592: F, t58647: F, t6535: F, t68250: F, t68633: F, t68636: F, t68640: F, t68694: F, t68711: F, t68730: F, t68751: F, t68754: F, t68757: F, t69139: F, t12429: F, t12486: F, t12553: F, t16971: F, t17097: F, t17151: F, t20678: F, t20679: F, t3453: F, t3477: F, t3497: F, t3515: F, t3521: F, t45061: F, t45174: F, t5158: F, t6487: F, t6503: F, t6519: F, t68760: F, t68763: F, t68766: F, t68769: F, t68772: F, t68779: F, t68781: F, t68784: F, t68791: F, t68794: F, t43771: F, t43781: F, t43783: F, t45106: F, t45107: F, t68253: F, t68255: F, t68257: F, t68262: F, t68267: F, t68271: F, t68275: F, t68277: F, t68282: F, t68287: F, t68292: F, t68312: F, t68315: F, t68319: F, t68322: F, t68326: F, t68330: F, t43911: F, t68368: F, t68370: F, t68373: F, t68402: F, t68464: F, t58145: F, t58147: F, t68470: F, t68473: F, t68476: F, t68479: F, t68481: F, t68484: F, t68486: F, t68488: F, t68490: F, t68493: F, t68495: F, t68497: F, t58153: F, t58158: F, t58160: F, t58162: F, t58165: F, t58186: F, t68507: F, t68515: F, t68518: F, t68521: F, t68524: F, t58207: F, t68529: F, t68532: F, t68535: F, t68538: F, t68540: F, t68543: F, t68546: F, t68548: F, t68550: F, t68553: F, t68556: F, t68559: F, t68561: F) -> (F, F, F, F, F, F, F, F, F) {
        let t69153 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3659::<F>(t68297, t68301, t68305, t68310, t68332, t68334, t68336, t68342, t68347, t68350, t68353, t68357, t68360);
        let t69167 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3660::<F>(t56176, t56183, t56185, t56187, t56189, t56209, t56212, t56214, t56216, t56228, t68363, t68366);
        let t69181 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3661::<F>(t43865, t43888, t43890, t43892, t56230, t56236, t68389, t68393, t68397, t68399, t68454, t68456, t68459);
        let t69192 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3662::<F>(t1168, t1187, t12476, t16948, t16959, t16979, t16989, t435, t57944, t57972, t58237, t58259, t58300, t58345, t58592, t58647, t6535, t68250, t68633, t68636, t68640, t68694, t68711, t68730, t68751, t68754, t68757, t69139, t69153, t69167, t69181);
        let t69216 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3663::<F>(t12429, t12486, t12553, t16971, t17097, t17151, t20678, t20679, t3453, t3477, t3497, t3515, t3521, t45061, t45174, t5158, t6487, t6503, t6519, t6535, t68760, t68763, t68766, t68769, t68772, t68779, t68781, t68784, t68791, t68794);
        let t69230 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3664::<F>(t43771, t43781, t43783, t45106, t45107, t68253, t68255, t68257, t68262, t68267, t68271, t68275, t68277, t68282);
        let t69246 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3665::<F>(t68287, t68292, t68297, t68301, t68305, t68310, t68312, t68315, t68319, t68322, t68326, t68330, t68332, t68334, t68336);
        let t69263 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3666::<F>(t43911, t56176, t56183, t56185, t68342, t68347, t68350, t68353, t68357, t68360, t68363, t68366, t68368, t68370, t68373);
        let t69279 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3667::<F>(t56187, t56189, t56209, t56212, t56214, t56216, t56228, t56230, t56236, t68389, t68393, t68397, t68399, t68402, t68464);
        let t69296 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3668::<F>(t58145, t58147, t68470, t68473, t68476, t68479, t68481, t68484, t68486, t68488, t68490, t68493, t68495, t68497);
        let t69312 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3669::<F>(t43865, t43888, t43890, t43892, t58153, t58158, t58160, t58162, t58165, t58186, t68507, t68515, t68518, t68521, t68524);
        let t69329 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3670::<F>(t58207, t68454, t68529, t68532, t68535, t68538, t68540, t68543, t68546, t68548, t68550, t68553, t68556, t68559, t68561);
    (t69192, t69216, t69230, t69246, t69263, t69279, t69296, t69312, t69329)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1042 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3636;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3637;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3638;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3639;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3640;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3641;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3642;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3643;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3644;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3645;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1042<F: Float>(t20887: F, t3531: F, t1196: F, t20886: F, t3516: F, t43771: F, t43781: F, t43783: F, t44039: F, t44040: F, t68253: F, t68255: F, t68257: F, t68262: F, t68267: F, t68271: F, t68275: F, t68277: F, t68282: F, t68287: F, t68292: F, t68297: F, t68301: F, t68305: F, t68310: F, t68312: F, t68315: F, t68319: F, t68322: F, t68326: F, t68330: F, t68332: F, t68334: F, t68336: F, t43911: F, t56176: F, t56183: F, t56185: F, t68342: F, t68347: F, t68350: F, t68353: F, t68357: F, t68360: F, t68363: F, t68366: F, t68368: F, t68370: F, t68373: F, t56187: F, t56189: F, t56209: F, t56212: F, t56214: F, t56216: F, t56228: F, t56230: F, t56236: F, t68389: F, t68393: F, t68397: F, t68399: F, t68402: F, t68464: F, t58145: F, t58147: F, t68470: F, t68473: F, t68476: F, t68479: F, t68481: F, t68484: F, t68486: F, t68488: F, t68490: F, t68493: F, t68495: F, t68497: F, t43865: F, t43888: F, t43890: F, t43892: F, t58153: F, t58158: F, t58160: F, t58162: F, t58165: F, t58186: F, t68507: F, t68515: F, t68518: F, t68521: F, t68524: F, t58207: F, t68454: F, t68529: F, t68532: F, t68535: F, t68538: F, t68540: F, t68543: F, t68546: F, t68548: F, t68550: F, t68553: F, t68556: F, t68559: F, t68561: F, t58209: F, t58211: F, t58225: F, t68456: F, t68459: F, t68567: F, t68570: F, t68573: F, t68576: F, t68578: F, t68583: F, t68585: F, t68588: F, t68590: F, t68593: F, t1131: F, t1150: F, t68779: F, t68781: F, t68784: F, t68786: F, t68789: F, t68791: F, t68794: F, t68799: F, t68803: F, t3385: F, t3433: F, t6471: F, t1130: F, t20469: F, t1151: F, t20629: F, t3428: F, t3432: F, t6433: F, t3436: F, t1733: F, t58460: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t68805, t68808, t68821) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3636::<F>(t20887, t3531, t1196, t20886, t3516, t43771, t43781, t43783, t44039, t44040, t68253, t68255, t68257, t68262, t68267, t68271, t68275, t68277, t68282);
        let t68837 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3637::<F>(t68287, t68292, t68297, t68301, t68305, t68310, t68312, t68315, t68319, t68322, t68326, t68330, t68332, t68334, t68336);
        let t68854 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3638::<F>(t43911, t56176, t56183, t56185, t68342, t68347, t68350, t68353, t68357, t68360, t68363, t68366, t68368, t68370, t68373);
        let t68870 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3639::<F>(t56187, t56189, t56209, t56212, t56214, t56216, t56228, t56230, t56236, t68389, t68393, t68397, t68399, t68402, t68464);
        let t68887 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3640::<F>(t58145, t58147, t68470, t68473, t68476, t68479, t68481, t68484, t68486, t68488, t68490, t68493, t68495, t68497);
        let t68903 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3641::<F>(t43865, t43888, t43890, t43892, t58153, t58158, t58160, t58162, t58165, t58186, t68507, t68515, t68518, t68521, t68524);
        let t68920 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3642::<F>(t58207, t68454, t68529, t68532, t68535, t68538, t68540, t68543, t68546, t68548, t68550, t68553, t68556, t68559, t68561);
        let t68936 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3643::<F>(t58209, t58211, t58225, t68456, t68459, t68567, t68570, t68573, t68576, t68578, t68583, t68585, t68588, t68590, t68593);
        let (t68942, t68943) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3644::<F>(t1131, t1150, t68821, t68837, t68854, t68870, t68887, t68903, t68920, t68936, t68779, t68781, t68784, t68786, t68789, t68791, t68794, t68799, t68803, t68805, t68808);
        let (t68946, t68949, t68951, t68954, t68956) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3645::<F>(t3385, t3433, t6471, t1130, t20469, t1151, t20629, t3428, t3432, t6433, t3436, t1733, t58460);
    (t68805, t68808, t68942, t68943, t68946, t68949, t68951, t68954, t68956)
}

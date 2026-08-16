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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1042(t20887: f64, t3531: f64, t1196: f64, t20886: f64, t3516: f64, t43771: f64, t43781: f64, t43783: f64, t44039: f64, t44040: f64, t68253: f64, t68255: f64, t68257: f64, t68262: f64, t68267: f64, t68271: f64, t68275: f64, t68277: f64, t68282: f64, t68287: f64, t68292: f64, t68297: f64, t68301: f64, t68305: f64, t68310: f64, t68312: f64, t68315: f64, t68319: f64, t68322: f64, t68326: f64, t68330: f64, t68332: f64, t68334: f64, t68336: f64, t43911: f64, t56176: f64, t56183: f64, t56185: f64, t68342: f64, t68347: f64, t68350: f64, t68353: f64, t68357: f64, t68360: f64, t68363: f64, t68366: f64, t68368: f64, t68370: f64, t68373: f64, t56187: f64, t56189: f64, t56209: f64, t56212: f64, t56214: f64, t56216: f64, t56228: f64, t56230: f64, t56236: f64, t68389: f64, t68393: f64, t68397: f64, t68399: f64, t68402: f64, t68464: f64, t58145: f64, t58147: f64, t68470: f64, t68473: f64, t68476: f64, t68479: f64, t68481: f64, t68484: f64, t68486: f64, t68488: f64, t68490: f64, t68493: f64, t68495: f64, t68497: f64, t43865: f64, t43888: f64, t43890: f64, t43892: f64, t58153: f64, t58158: f64, t58160: f64, t58162: f64, t58165: f64, t58186: f64, t68507: f64, t68515: f64, t68518: f64, t68521: f64, t68524: f64, t58207: f64, t68454: f64, t68529: f64, t68532: f64, t68535: f64, t68538: f64, t68540: f64, t68543: f64, t68546: f64, t68548: f64, t68550: f64, t68553: f64, t68556: f64, t68559: f64, t68561: f64, t58209: f64, t58211: f64, t58225: f64, t68456: f64, t68459: f64, t68567: f64, t68570: f64, t68573: f64, t68576: f64, t68578: f64, t68583: f64, t68585: f64, t68588: f64, t68590: f64, t68593: f64, t1131: f64, t1150: f64, t68779: f64, t68781: f64, t68784: f64, t68786: f64, t68789: f64, t68791: f64, t68794: f64, t68799: f64, t68803: f64, t3385: f64, t3433: f64, t6471: f64, t1130: f64, t20469: f64, t1151: f64, t20629: f64, t3428: f64, t3432: f64, t6433: f64, t3436: f64, t1733: f64, t58460: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t68805, t68808, t68821) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3636(t20887, t3531, t1196, t20886, t3516, t43771, t43781, t43783, t44039, t44040, t68253, t68255, t68257, t68262, t68267, t68271, t68275, t68277, t68282);
        let t68837 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3637(t68287, t68292, t68297, t68301, t68305, t68310, t68312, t68315, t68319, t68322, t68326, t68330, t68332, t68334, t68336);
        let t68854 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3638(t43911, t56176, t56183, t56185, t68342, t68347, t68350, t68353, t68357, t68360, t68363, t68366, t68368, t68370, t68373);
        let t68870 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3639(t56187, t56189, t56209, t56212, t56214, t56216, t56228, t56230, t56236, t68389, t68393, t68397, t68399, t68402, t68464);
        let t68887 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3640(t58145, t58147, t68470, t68473, t68476, t68479, t68481, t68484, t68486, t68488, t68490, t68493, t68495, t68497);
        let t68903 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3641(t43865, t43888, t43890, t43892, t58153, t58158, t58160, t58162, t58165, t58186, t68507, t68515, t68518, t68521, t68524);
        let t68920 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3642(t58207, t68454, t68529, t68532, t68535, t68538, t68540, t68543, t68546, t68548, t68550, t68553, t68556, t68559, t68561);
        let t68936 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3643(t58209, t58211, t58225, t68456, t68459, t68567, t68570, t68573, t68576, t68578, t68583, t68585, t68588, t68590, t68593);
        let (t68942, t68943) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3644(t1131, t1150, t68821, t68837, t68854, t68870, t68887, t68903, t68920, t68936, t68779, t68781, t68784, t68786, t68789, t68791, t68794, t68799, t68803, t68805, t68808);
        let (t68946, t68949, t68951, t68954, t68956) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3645(t3385, t3433, t6471, t1130, t20469, t1151, t20629, t3428, t3432, t6433, t3436, t1733, t58460);
    (t68805, t68808, t68942, t68943, t68946, t68949, t68951, t68954, t68956)
}

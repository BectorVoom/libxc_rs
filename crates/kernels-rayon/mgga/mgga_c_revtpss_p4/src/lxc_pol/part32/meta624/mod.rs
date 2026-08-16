//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta624 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1968;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1969;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1970;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1971;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1972;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1973;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1974;
use chunk7::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1975;
use chunk8::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1976;
use chunk9::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1977;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta624(t1448: f64, t6922: f64, t7897: f64, t8995: f64, t101448: f64, t101451: f64, t101755: f64, t101756: f64, t105870: f64, t105873: f64, t105876: f64, t105878: f64, t105881: f64, t105883: f64, t95397: f64, t114: f64, t108138: f64, t96187: f64, t96236: f64, t30256: f64, t689: f64, t25904: f64, t102081: f64, t102084: f64, t102086: f64, t102090: f64, t102093: f64, t102096: f64, t102098: f64, t102101: f64, t102104: f64, t102113: f64, t96197: f64, t25899: f64, t30278: f64, t686: f64, t72: f64, t94674: f64, t30295: f64, t7284: f64, t30282: f64, t25895: f64, t6919: f64, t7492: f64, t102117: f64, t102120: f64, t102122: f64, t102129: f64, t102131: f64, t102133: f64, t102135: f64, t102139: f64, t96206: f64, t30266: f64, t25878: f64, t94669: f64, t102143: f64, t102164: f64, t102167: f64, t1398: f64, t27837: f64, t28830: f64, t30247: f64, t543: f64, t5658: f64, t7295: f64, t7301: f64, t8085: f64, t96210: f64, t96211: f64, t96218: f64, t96222: f64, t96230: f64, t30308: f64, t30261: f64, t102205: f64, t102213: f64, t102217: f64, t102219: f64, t102225: f64, t102237: f64, t102239: f64, t28841: f64, t96246: f64, t96253: f64, t1358: f64, t212: f64, t102241: f64, t102246: f64, t102249: f64, t102253: f64, t108225: f64, t108371: f64, t108502: f64, t1882: f64, t2103: f64, t25921: f64, t25930: f64, t28855: f64, t28888: f64, t28911: f64, t30262: f64, t8095: f64, t96257: f64, t96260: f64, t96265: f64, t98050: f64, t1904: f64, t28824: f64, t7289: f64, t27884: f64, t28845: f64, t102255: f64, t102257: f64, t102261: f64, t102266: f64, t102270: f64, t102272: f64, t102276: f64, t108653: f64, t25924: f64, t26304: f64, t27868: f64, t28792: f64, t5774: f64, t75016: f64, t8094: f64, t94823: f64, t96277: f64, t102293: f64, t102296: f64, t102298: f64, t102306: f64, t102309: f64, t102316: f64, t108178: f64, t28008: f64, t28915: f64, t8104: f64, t96280: f64, t96284: f64, t96287: f64, t96289: f64, t96298: f64, t97933: f64, t102420: f64, t5722: f64, t28780: f64, t98041: f64, t27899: f64, t28894: f64, t97802: f64, t98380: f64, t102320: f64, t102324: f64, t102325: f64, t102656: f64, t108244: f64, t14224: f64, t1444: f64, t28806: f64, t30279: f64, t6895: f64, t7506: f64, t96374: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t109263, t109269, t109367) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1968(t1448, t6922, t7897, t8995, t101448, t101451, t101755, t101756, t105870, t105873, t105876, t105878, t105881, t105883, t95397);
        let (t109368, t109396, t109399) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1969(t114, t109367, t108138, t96187, t96236, t30256, t689, t25904, t102081, t102084, t102086, t102090, t102093, t102096, t102098, t102101, t102104, t102113, t96197);
        let (t109400, t109403, t109404, t109407, t109408, t109412, t109413, t109417) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1970(t109396, t25899, t30278, t686, t72, t94674, t30295, t7284, t30282, t25895, t689, t6919, t7492);
        let t109423 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1971(t102117, t102120, t102122, t102129, t102131, t102133, t102135, t102139, t109400, t109404, t109408, t109413, t109417, t96206);
        let (t109425, t109446) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1972(t30266, t689, t25904, t109412, t25878, t109403, t94669, t102143, t102164, t102167, t1398, t27837, t28830, t30247, t543, t5658, t7295, t7301, t8085, t96210, t96211, t96218, t96222, t96230);
        let t109467 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1973(t30308, t686, t72, t25895, t25878, t109425, t25899, t30261, t689, t25904, t102205, t102213, t102217, t102219, t102225, t102237, t102239, t27837, t28841, t96246, t96253);
        let t109493 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1974(t1358, t212, t30247, t689, t102241, t102246, t102249, t102253, t108225, t108371, t108502, t1882, t2103, t25921, t25930, t28855, t28888, t28911, t30262, t543, t7295, t7301, t8095, t96257, t96260, t96265, t98050);
        let t109516 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1975(t1904, t28824, t689, t109407, t7289, t27884, t28845, t102255, t102257, t102261, t102266, t102270, t102272, t102276, t108653, t25924, t26304, t27837, t27868, t28792, t5774, t7295, t75016, t8094, t94823, t96277);
        let t109533 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1976(t102293, t102296, t102298, t102306, t102309, t102316, t108178, t25930, t26304, t28008, t28915, t8104, t96280, t96284, t96287, t96289, t96298, t97933);
        let t109563 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1977(t102420, t5722, t28780, t98041, t27899, t28845, t28894, t97802, t98380, t102320, t102324, t102325, t102656, t108244, t14224, t1444, t25921, t25924, t25930, t26304, t27837, t27868, t28806, t30279, t30282, t6895, t7295, t7506, t96374);
    (t109263, t109269, t109368, t109399, t109423, t109446, t109467, t109493, t109516, t109533, t109563)
}

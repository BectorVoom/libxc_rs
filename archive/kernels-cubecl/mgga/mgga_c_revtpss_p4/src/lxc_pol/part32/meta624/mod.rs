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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta624<F: Float>(t1448: F, t6922: F, t7897: F, t8995: F, t101448: F, t101451: F, t101755: F, t101756: F, t105870: F, t105873: F, t105876: F, t105878: F, t105881: F, t105883: F, t95397: F, t114: F, t108138: F, t96187: F, t96236: F, t30256: F, t689: F, t25904: F, t102081: F, t102084: F, t102086: F, t102090: F, t102093: F, t102096: F, t102098: F, t102101: F, t102104: F, t102113: F, t96197: F, t25899: F, t30278: F, t686: F, t72: F, t94674: F, t30295: F, t7284: F, t30282: F, t25895: F, t6919: F, t7492: F, t102117: F, t102120: F, t102122: F, t102129: F, t102131: F, t102133: F, t102135: F, t102139: F, t96206: F, t30266: F, t25878: F, t94669: F, t102143: F, t102164: F, t102167: F, t1398: F, t27837: F, t28830: F, t30247: F, t543: F, t5658: F, t7295: F, t7301: F, t8085: F, t96210: F, t96211: F, t96218: F, t96222: F, t96230: F, t30308: F, t30261: F, t102205: F, t102213: F, t102217: F, t102219: F, t102225: F, t102237: F, t102239: F, t28841: F, t96246: F, t96253: F, t1358: F, t212: F, t102241: F, t102246: F, t102249: F, t102253: F, t108225: F, t108371: F, t108502: F, t1882: F, t2103: F, t25921: F, t25930: F, t28855: F, t28888: F, t28911: F, t30262: F, t8095: F, t96257: F, t96260: F, t96265: F, t98050: F, t1904: F, t28824: F, t7289: F, t27884: F, t28845: F, t102255: F, t102257: F, t102261: F, t102266: F, t102270: F, t102272: F, t102276: F, t108653: F, t25924: F, t26304: F, t27868: F, t28792: F, t5774: F, t75016: F, t8094: F, t94823: F, t96277: F, t102293: F, t102296: F, t102298: F, t102306: F, t102309: F, t102316: F, t108178: F, t28008: F, t28915: F, t8104: F, t96280: F, t96284: F, t96287: F, t96289: F, t96298: F, t97933: F, t102420: F, t5722: F, t28780: F, t98041: F, t27899: F, t28894: F, t97802: F, t98380: F, t102320: F, t102324: F, t102325: F, t102656: F, t108244: F, t14224: F, t1444: F, t28806: F, t30279: F, t6895: F, t7506: F, t96374: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t109263, t109269, t109367) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1968::<F>(t1448, t6922, t7897, t8995, t101448, t101451, t101755, t101756, t105870, t105873, t105876, t105878, t105881, t105883, t95397);
        let (t109368, t109396, t109399) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1969::<F>(t114, t109367, t108138, t96187, t96236, t30256, t689, t25904, t102081, t102084, t102086, t102090, t102093, t102096, t102098, t102101, t102104, t102113, t96197);
        let (t109400, t109403, t109404, t109407, t109408, t109412, t109413, t109417) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1970::<F>(t109396, t25899, t30278, t686, t72, t94674, t30295, t7284, t30282, t25895, t689, t6919, t7492);
        let t109423 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1971::<F>(t102117, t102120, t102122, t102129, t102131, t102133, t102135, t102139, t109400, t109404, t109408, t109413, t109417, t96206);
        let (t109425, t109446) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1972::<F>(t30266, t689, t25904, t109412, t25878, t109403, t94669, t102143, t102164, t102167, t1398, t27837, t28830, t30247, t543, t5658, t7295, t7301, t8085, t96210, t96211, t96218, t96222, t96230);
        let t109467 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1973::<F>(t30308, t686, t72, t25895, t25878, t109425, t25899, t30261, t689, t25904, t102205, t102213, t102217, t102219, t102225, t102237, t102239, t27837, t28841, t96246, t96253);
        let t109493 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1974::<F>(t1358, t212, t30247, t689, t102241, t102246, t102249, t102253, t108225, t108371, t108502, t1882, t2103, t25921, t25930, t28855, t28888, t28911, t30262, t543, t7295, t7301, t8095, t96257, t96260, t96265, t98050);
        let t109516 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1975::<F>(t1904, t28824, t689, t109407, t7289, t27884, t28845, t102255, t102257, t102261, t102266, t102270, t102272, t102276, t108653, t25924, t26304, t27837, t27868, t28792, t5774, t7295, t75016, t8094, t94823, t96277);
        let t109533 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1976::<F>(t102293, t102296, t102298, t102306, t102309, t102316, t108178, t25930, t26304, t28008, t28915, t8104, t96280, t96284, t96287, t96289, t96298, t97933);
        let t109563 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1977::<F>(t102420, t5722, t28780, t98041, t27899, t28845, t28894, t97802, t98380, t102320, t102324, t102325, t102656, t108244, t14224, t1444, t25921, t25924, t25930, t26304, t27837, t27868, t28806, t30279, t30282, t6895, t7295, t7506, t96374);
    (t109263, t109269, t109368, t109399, t109423, t109446, t109467, t109493, t109516, t109533, t109563)
}

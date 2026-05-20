//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta626 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1990;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1991;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1992;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1993;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1994;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1995;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1996;
use chunk7::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1997;
use chunk8::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1998;
use chunk9::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1999;
use chunk10::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2000;
use chunk11::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2001;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta626<F: Float>(t102528: F, t102530: F, t102531: F, t102534: F, t102535: F, t102537: F, t102548: F, t108590: F, t108592: F, t94498: F, t96326: F, t98224: F, t98260: F, t102557: F, t108601: F, t108604: F, t108606: F, t108608: F, t94546: F, t96341: F, t96342: F, t98263: F, t98264: F, t98267: F, t98268: F, t102567: F, t108615: F, t108617: F, t108619: F, t108623: F, t108625: F, t108627: F, t108629: F, t94554: F, t96358: F, t96359: F, t98283: F, t98285: F, t109777: F, t109788: F, t109798: F, t109808: F, t109816: F, t22399: F, t26265: F, t2027: F, t2028: F, t213: F, t225: F, t25921: F, t26282: F, t28890: F, t28899: F, t30283: F, t543: F, t545: F, t561: F, t5774: F, t5775: F, t6843: F, t6919: F, t7295: F, t7296: F, t7301: F, t7506: F, t7917: F, t8085: F, t96559: F, t96561: F, t96564: F, t96565: F, t96584: F, t96591: F, t1353: F, t2106: F, t101970: F, t28154: F, t101782: F, t101783: F, t101790: F, t101793: F, t101811: F, t101820: F, t108941: F, t1923: F, t2047: F, t28093: F, t28635: F, t30543: F, t6954: F, t7702: F, t7964: F, t95246: F, t108879: F, t101237: F, t101240: F, t101850: F, t108872: F, t108876: F, t108945: F, t108952: F, t2048: F, t26175: F, t28628: F, t29513: F, t29551: F, t7352: F, t92568: F, t95253: F, t95255: F, t95316: F, t60673: F, t7342: F, t101243: F, t101935: F, t101938: F, t108762: F, t108769: F, t108792: F, t108864: F, t28133: F, t28141: F, t28602: F, t29562: F, t6960: F, t6963: F, t7343: F, t95276: F, t101886: F, t108733: F, t108737: F, t108745: F, t108807: F, t108810: F, t108813: F, t26187: F, t28105: F, t28109: F, t29538: F, t29544: F, t29548: F, t7706: F, t2247: F, t5819: F, t68: F, t1469: F, t603: F, t7349: F, t28640: F, t29532: F, t7348: F, t101870: F, t101872: F, t101874: F, t101879: F, t101881: F, t108749: F, t108759: F, t101788: F, t101883: F, t101885: F, t108765: F, t108816: F, t28112: F, t28116: F, t28119: F, t29554: F, t7709: F, t95294: F, t26179: F, t95319: F, t101899: F, t101901: F, t101903: F, t101906: F, t101907: F, t101929: F, t95314: F) -> (F, F, F, F, F, F, F, F, F) {
        let t109822 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1990::<F>(t102528, t102530, t102531, t102534, t102535, t102537, t102548, t108590, t108592, t94498, t96326, t98224, t98260);
        let t109829 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1991::<F>(t102557, t108601, t108604, t108606, t108608, t94546, t96341, t96342, t98263, t98264, t98267, t98268);
        let t109839 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1992::<F>(t102567, t108615, t108617, t108619, t108623, t108625, t108627, t108629, t94554, t96358, t96359, t98283, t98285);
        let t109864 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1993::<F>(t109777, t109788, t109798, t109808, t109816, t109822, t109829, t109839, t22399, t26265, t2027, t2028, t213, t225, t25921, t26282, t28890, t28899, t30283, t543, t545, t561, t5774, t5775, t6843, t6919, t7295, t7296, t7301, t7506, t7917, t8085, t96559, t96561, t96564, t96565, t96584, t96591);
        let (t109874, t109895) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1994::<F>(t1353, t2106, t101970, t28154, t101782, t101783, t101790, t101793, t101811, t101820, t108941, t1923, t2047, t28093, t28635, t30543, t6954, t7702, t7964, t95246);
        let t109918 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1995::<F>(t108879, t2047, t101237, t101240, t101850, t108872, t108876, t108945, t108952, t2048, t26175, t28154, t28628, t29513, t29551, t7352, t92568, t95253, t95255, t95316);
        let t109945 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1996::<F>(t60673, t7342, t101243, t101935, t101938, t108762, t108769, t108792, t108864, t2048, t26175, t28133, t28141, t28154, t28602, t28628, t29562, t30543, t6960, t6963, t7343, t7964, t95276);
        let t109970 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1997::<F>(t101886, t108733, t108737, t108745, t108807, t108810, t108813, t2048, t26187, t28105, t28109, t28602, t29538, t29544, t29548, t7343, t7352, t7706);
        let (t109976, t109980, t109983, t109985, t109988) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1998::<F>(t2247, t5819, t68, t1469, t603, t29513, t7349, t28640, t7702, t1923, t29532, t7348);
        let t109992 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1999::<F>(t29551, t7349, t101870, t101872, t101874, t101879, t101881, t108749, t108759, t109976, t109980, t109983, t109985, t109988, t6960, t7343);
        let t110012 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2000::<F>(t101788, t7706, t29538, t7349, t101883, t101885, t108765, t108816, t2048, t28112, t28116, t28119, t28635, t29554, t7352, t7709, t7964, t95294);
        let t110027 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2001::<F>(t26179, t29544, t29548, t29554, t7349, t28640, t7709, t29562, t95319, t101899, t101901, t101903, t101906, t101907, t101929, t95314);
    (t109864, t109874, t109895, t109918, t109945, t109970, t109992, t110012, t110027)
}

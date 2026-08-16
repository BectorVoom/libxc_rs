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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta626(t102528: f64, t102530: f64, t102531: f64, t102534: f64, t102535: f64, t102537: f64, t102548: f64, t108590: f64, t108592: f64, t94498: f64, t96326: f64, t98224: f64, t98260: f64, t102557: f64, t108601: f64, t108604: f64, t108606: f64, t108608: f64, t94546: f64, t96341: f64, t96342: f64, t98263: f64, t98264: f64, t98267: f64, t98268: f64, t102567: f64, t108615: f64, t108617: f64, t108619: f64, t108623: f64, t108625: f64, t108627: f64, t108629: f64, t94554: f64, t96358: f64, t96359: f64, t98283: f64, t98285: f64, t109777: f64, t109788: f64, t109798: f64, t109808: f64, t109816: f64, t22399: f64, t26265: f64, t2027: f64, t2028: f64, t213: f64, t225: f64, t25921: f64, t26282: f64, t28890: f64, t28899: f64, t30283: f64, t543: f64, t545: f64, t561: f64, t5774: f64, t5775: f64, t6843: f64, t6919: f64, t7295: f64, t7296: f64, t7301: f64, t7506: f64, t7917: f64, t8085: f64, t96559: f64, t96561: f64, t96564: f64, t96565: f64, t96584: f64, t96591: f64, t1353: f64, t2106: f64, t101970: f64, t28154: f64, t101782: f64, t101783: f64, t101790: f64, t101793: f64, t101811: f64, t101820: f64, t108941: f64, t1923: f64, t2047: f64, t28093: f64, t28635: f64, t30543: f64, t6954: f64, t7702: f64, t7964: f64, t95246: f64, t108879: f64, t101237: f64, t101240: f64, t101850: f64, t108872: f64, t108876: f64, t108945: f64, t108952: f64, t2048: f64, t26175: f64, t28628: f64, t29513: f64, t29551: f64, t7352: f64, t92568: f64, t95253: f64, t95255: f64, t95316: f64, t60673: f64, t7342: f64, t101243: f64, t101935: f64, t101938: f64, t108762: f64, t108769: f64, t108792: f64, t108864: f64, t28133: f64, t28141: f64, t28602: f64, t29562: f64, t6960: f64, t6963: f64, t7343: f64, t95276: f64, t101886: f64, t108733: f64, t108737: f64, t108745: f64, t108807: f64, t108810: f64, t108813: f64, t26187: f64, t28105: f64, t28109: f64, t29538: f64, t29544: f64, t29548: f64, t7706: f64, t2247: f64, t5819: f64, t68: f64, t1469: f64, t603: f64, t7349: f64, t28640: f64, t29532: f64, t7348: f64, t101870: f64, t101872: f64, t101874: f64, t101879: f64, t101881: f64, t108749: f64, t108759: f64, t101788: f64, t101883: f64, t101885: f64, t108765: f64, t108816: f64, t28112: f64, t28116: f64, t28119: f64, t29554: f64, t7709: f64, t95294: f64, t26179: f64, t95319: f64, t101899: f64, t101901: f64, t101903: f64, t101906: f64, t101907: f64, t101929: f64, t95314: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t109822 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1990(t102528, t102530, t102531, t102534, t102535, t102537, t102548, t108590, t108592, t94498, t96326, t98224, t98260);
        let t109829 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1991(t102557, t108601, t108604, t108606, t108608, t94546, t96341, t96342, t98263, t98264, t98267, t98268);
        let t109839 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1992(t102567, t108615, t108617, t108619, t108623, t108625, t108627, t108629, t94554, t96358, t96359, t98283, t98285);
        let t109864 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1993(t109777, t109788, t109798, t109808, t109816, t109822, t109829, t109839, t22399, t26265, t2027, t2028, t213, t225, t25921, t26282, t28890, t28899, t30283, t543, t545, t561, t5774, t5775, t6843, t6919, t7295, t7296, t7301, t7506, t7917, t8085, t96559, t96561, t96564, t96565, t96584, t96591);
        let (t109874, t109895) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1994(t1353, t2106, t101970, t28154, t101782, t101783, t101790, t101793, t101811, t101820, t108941, t1923, t2047, t28093, t28635, t30543, t6954, t7702, t7964, t95246);
        let t109918 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1995(t108879, t2047, t101237, t101240, t101850, t108872, t108876, t108945, t108952, t2048, t26175, t28154, t28628, t29513, t29551, t7352, t92568, t95253, t95255, t95316);
        let t109945 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1996(t60673, t7342, t101243, t101935, t101938, t108762, t108769, t108792, t108864, t2048, t26175, t28133, t28141, t28154, t28602, t28628, t29562, t30543, t6960, t6963, t7343, t7964, t95276);
        let t109970 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1997(t101886, t108733, t108737, t108745, t108807, t108810, t108813, t2048, t26187, t28105, t28109, t28602, t29538, t29544, t29548, t7343, t7352, t7706);
        let (t109976, t109980, t109983, t109985, t109988) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1998(t2247, t5819, t68, t1469, t603, t29513, t7349, t28640, t7702, t1923, t29532, t7348);
        let t109992 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1999(t29551, t7349, t101870, t101872, t101874, t101879, t101881, t108749, t108759, t109976, t109980, t109983, t109985, t109988, t6960, t7343);
        let t110012 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2000(t101788, t7706, t29538, t7349, t101883, t101885, t108765, t108816, t2048, t28112, t28116, t28119, t28635, t29554, t7352, t7709, t7964, t95294);
        let t110027 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2001(t26179, t29544, t29548, t29554, t7349, t28640, t7709, t29562, t95319, t101899, t101901, t101903, t101906, t101907, t101929, t95314);
    (t109864, t109874, t109895, t109918, t109945, t109970, t109992, t110012, t110027)
}

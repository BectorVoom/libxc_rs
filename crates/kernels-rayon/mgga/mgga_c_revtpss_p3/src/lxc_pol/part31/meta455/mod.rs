//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta455 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1637;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1638;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1639;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1640;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1641;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1642;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1643;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1644;
use chunk8::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1645;
use chunk9::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1646;
use chunk10::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1647;
use chunk11::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1648;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta455(t17661: f64, t5401: f64, t1214: f64, t1715: f64, t1250: f64, t17353: f64, t5052: f64, t5406: f64, t1794: f64, t3617: f64, t372: f64, t5047: f64, t3603: f64, t5284: f64, t5332: f64, t3720: f64, t12866: f64, t17340: f64, t17342: f64, t17693: f64, t17729: f64, t20914: f64, t20917: f64, t20923: f64, t20927: f64, t3711: f64, t5340: f64, t11249: f64, t6628: f64, t1248: f64, t13045: f64, t5341: f64, t1219: f64, t6667: f64, t247: f64, t3634: f64, t6429: f64, t1261: f64, t12856: f64, t20795: f64, t19666: f64, t5268: f64, t1042: f64, t17202: f64, t19661: f64, t12855: f64, t12967: f64, t17362: f64, t17569: f64, t17709: f64, t17747: f64, t3647: f64, t5299: f64, t5391: f64, t5397: f64, t6611: f64, t6679: f64, t5378: f64, t17459: f64, t6688: f64, t5405: f64, t6421: f64, t12787: f64, t17394: f64, t4890: f64, t3767: f64, t3782: f64, t3628: f64, t4186: f64, t5351: f64, t3626: f64, t12910: f64, t17283: f64, t17375: f64, t17448: f64, t17605: f64, t1791: f64, t3625: f64, t5320: f64, t5323: f64, t5335: f64, t5343: f64, t5402: f64, t5407: f64, t12712: f64, t471: f64, t1774: f64, t3367: f64, t4181: f64, t6622: f64, t73: f64, t5352: f64, t5333: f64, t17934: f64, t5330: f64, t5327: f64, t5362: f64, t12809: f64, t12853: f64, t17290: f64, t17386: f64, t17417: f64, t17425: f64, t17753: f64, t3718: f64, t1803: f64, t5326: f64, t12297: f64, t12610: f64, t16706: f64, t16708: f64, t16711: f64, t16713: f64, t20283: f64, t20285: f64, t20287: f64, t20290: f64, t20295: f64, t20300: f64, t20304: f64, t20308: f64, t20312: f64, t20315: f64, t20320: f64, t482: f64, t371: f64, t12772: f64, t6639: f64, t1263: f64, t6573: f64, t1122: f64, t1038: f64, t6593: f64, t1244: f64, t1241: f64, t5273: f64, t5292: f64, t17235: f64, t1235: f64, t1238: f64, t1252: f64, t17505: f64, t3667: f64, t5279: f64, t5384: f64, t6647: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20929, t20934, t20938, t20941, t20945, t20946) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1637(t17661, t5401, t1214, t1715, t1250, t17353, t5052, t5406, t1794, t3617, t372, t5047);
        let t20955 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1638(t20945, t20946, t3603, t5284, t5332, t3720, t12866, t17340, t17342, t17693, t17729, t20914, t20917, t20923, t20927, t20929, t20934, t20938, t20941, t3711, t5340);
        let t20956 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1639(t11249, t6628);
        let (t20959, t20963, t20966, t20974, t20977) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1640(t1248, t13045, t20956, t3720, t5341, t1219, t6667, t247, t3634, t6429, t1261, t12856, t20795);
        let t20993 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1641(t20977, t3720, t19666, t5268, t1042, t17202, t19661, t1261, t12855, t12967, t17362, t17569, t17709, t17747, t20959, t20963, t20966, t20974, t3647, t5299, t5391, t5397, t6611, t6679);
        let (t21001, t21004, t21008, t21014, t21017) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1642(t5378, t5391, t17459, t6688, t3720, t5405, t6421, t12787, t17394, t4890, t3767, t3782);
        let t21027 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1643(t3628, t4186, t5351, t3626, t12910, t17283, t17375, t17448, t17605, t1791, t21001, t21004, t21008, t21014, t21017, t3625, t5320, t5323, t5335, t5343, t5402, t5407);
        let (t21030, t21037, t21040, t21042, t21045) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1644(t12712, t471, t6688, t3720, t1774, t3367, t4181, t3626, t6622, t73, t5352, t20956, t5333);
        let t21057 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1645(t21045, t3720, t17934, t5330, t5327, t5362, t12809, t12853, t17290, t17386, t17417, t17425, t17605, t17729, t17753, t1791, t21030, t21037, t21042, t3718, t5343, t5402);
        let (t21063, t21082) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1646(t1803, t5326, t12297, t12610, t16706, t16708, t16711, t16713, t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320);
        let (t21085, t21088, t21091, t21094) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1647(t21082, t482, t371, t372, t5323, t5362, t12772, t6639, t3625, t1263, t6573, t1122);
        let t21114 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1648(t1042, t21094, t1038, t6593, t1244, t1241, t5273, t5292, t17235, t19661, t1235, t1238, t1252, t1261, t17505, t17569, t21063, t21085, t21088, t21091, t3667, t5279, t5320, t5327, t5384, t6647);
    (t20955, t20956, t20993, t21027, t21040, t21057, t21082, t21114)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta433 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1541;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1542;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1543;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1544;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1545;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1546;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1547;
use chunk7::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1548;
use chunk8::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1549;
use chunk9::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1550;
use chunk10::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1551;
use chunk11::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1552;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta433<F: Float>(t17661: F, t5401: F, t1214: F, t1715: F, t1250: F, t17353: F, t5052: F, t5406: F, t1794: F, t3617: F, t372: F, t5047: F, t3603: F, t5284: F, t5332: F, t3720: F, t12866: F, t17340: F, t17342: F, t17693: F, t17729: F, t20914: F, t20917: F, t20923: F, t20927: F, t3711: F, t5340: F, t11249: F, t6628: F, t1248: F, t13045: F, t5341: F, t1219: F, t6667: F, t247: F, t3634: F, t6429: F, t1261: F, t12856: F, t20795: F, t19666: F, t5268: F, t1042: F, t17202: F, t19661: F, t12855: F, t12967: F, t17362: F, t17569: F, t17709: F, t17747: F, t3647: F, t5299: F, t5391: F, t5397: F, t6611: F, t6679: F, t5378: F, t17459: F, t6688: F, t5405: F, t6421: F, t12787: F, t17394: F, t4890: F, t3767: F, t3782: F, t3628: F, t4186: F, t5351: F, t3626: F, t12910: F, t17283: F, t17375: F, t17448: F, t17605: F, t1791: F, t3625: F, t5320: F, t5323: F, t5335: F, t5343: F, t5402: F, t5407: F, t12712: F, t471: F, t1774: F, t3367: F, t4181: F, t6622: F, t73: F, t5352: F, t5333: F, t17934: F, t5330: F, t5327: F, t5362: F, t12809: F, t12853: F, t17290: F, t17386: F, t17417: F, t17425: F, t17753: F, t3718: F, t1803: F, t5326: F, t12297: F, t12610: F, t16706: F, t16708: F, t16711: F, t16713: F, t20283: F, t20285: F, t20287: F, t20290: F, t20295: F, t20300: F, t20304: F, t20308: F, t20312: F, t20315: F, t20320: F, t482: F, t371: F, t12772: F, t6639: F, t1263: F, t6573: F, t1122: F, t1038: F, t6593: F, t1244: F, t1241: F, t5273: F, t5292: F, t17235: F, t1235: F, t1238: F, t1252: F, t17505: F, t3667: F, t5279: F, t5384: F, t6647: F) -> (F, F, F, F, F, F, F, F) {
        let (t20929, t20934, t20938, t20941, t20945, t20946) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1541::<F>(t17661, t5401, t1214, t1715, t1250, t17353, t5052, t5406, t1794, t3617, t372, t5047);
        let t20955 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1542::<F>(t20945, t20946, t3603, t5284, t5332, t3720, t12866, t17340, t17342, t17693, t17729, t20914, t20917, t20923, t20927, t20929, t20934, t20938, t20941, t3711, t5340);
        let t20956 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1543::<F>(t11249, t6628);
        let (t20959, t20963, t20966, t20974, t20977) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1544::<F>(t1248, t13045, t20956, t3720, t5341, t1219, t6667, t247, t3634, t6429, t1261, t12856, t20795);
        let t20993 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1545::<F>(t20977, t3720, t19666, t5268, t1042, t17202, t19661, t1261, t12855, t12967, t17362, t17569, t17709, t17747, t20959, t20963, t20966, t20974, t3647, t5299, t5391, t5397, t6611, t6679);
        let (t21001, t21004, t21008, t21014, t21017) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1546::<F>(t5378, t5391, t17459, t6688, t3720, t5405, t6421, t12787, t17394, t4890, t3767, t3782);
        let t21027 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1547::<F>(t3628, t4186, t5351, t3626, t12910, t17283, t17375, t17448, t17605, t1791, t21001, t21004, t21008, t21014, t21017, t3625, t5320, t5323, t5335, t5343, t5402, t5407);
        let (t21030, t21037, t21040, t21042, t21045) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1548::<F>(t12712, t471, t6688, t3720, t1774, t3367, t4181, t3626, t6622, t73, t5352, t20956, t5333);
        let t21057 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1549::<F>(t21045, t3720, t17934, t5330, t5327, t5362, t12809, t12853, t17290, t17386, t17417, t17425, t17605, t17729, t17753, t1791, t21030, t21037, t21042, t3718, t5343, t5402);
        let (t21063, t21082) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1550::<F>(t1803, t5326, t12297, t12610, t16706, t16708, t16711, t16713, t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320);
        let (t21085, t21088, t21091, t21094) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1551::<F>(t21082, t482, t371, t372, t5323, t5362, t12772, t6639, t3625, t1263, t6573, t1122);
        let t21114 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1552::<F>(t1042, t21094, t1038, t6593, t1244, t1241, t5273, t5292, t17235, t19661, t1235, t1238, t1252, t1261, t17505, t17569, t21063, t21085, t21088, t21091, t3667, t5279, t5320, t5327, t5384, t6647);
    (t20955, t20956, t20993, t21027, t21040, t21057, t21082, t21114)
}

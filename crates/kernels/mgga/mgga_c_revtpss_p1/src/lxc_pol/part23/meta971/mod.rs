//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta971 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3279;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3280;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3281;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3282;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3283;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3284;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3285;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3286;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3287;
use chunk9::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3288;
use chunk10::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3289;
use chunk11::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3290;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta971<F: Float>(t22953: F, t2661: F, t3992: F, t543: F, t550: F, t22857: F, t46609: F, t9994: F, t4003: F, t9934: F, t221: F, t22809: F, t3978: F, t3979: F, t22815: F, t3989: F, t22813: F, t46716: F, t1883: F, t22020: F, t22877: F, t46691: F, t22822: F, t1353: F, t1410: F, t1414: F, t22852: F, t49071: F, t49093: F, t74638: F, t74641: F, t74656: F, t74660: F, t74664: F, t828: F, t85442: F, t22912: F, t4018: F, t4019: F, t6869: F, t73920: F, t22245: F, t22841: F, t74026: F, t47337: F, t49087: F, t49090: F, t49103: F, t49105: F, t49122: F, t49125: F, t49127: F, t74677: F, t74682: F, t74698: F, t74711: F, t74714: F, t74717: F, t85562: F, t85623: F, t85680: F, t85709: F, t85738: F, t85780: F, t85830: F, t85854: F, t85871: F, t86086: F, t86106: F, t86136: F, t86162: F, t86198: F, t14100: F, t22399: F, t1904: F, t213: F, t22390: F, t225: F, t47504: F, t47512: F, t47886: F, t47899: F, t47904: F, t561: F, t5728: F, t73666: F, t73671: F, t73673: F, t73676: F, t73705: F, t73707: F, t74802: F, t85509: F, t5722: F, t74835: F, t1357: F, t23043: F, t689: F, t47561: F, t47907: F, t47920: F, t47932: F, t47938: F, t47942: F, t47945: F, t47948: F, t47953: F, t49468: F, t73712: F, t74733: F, t74744: F, t1364: F, t22965: F, t786: F, t22975: F, t5599: F, t6896: F, t1424: F, t1444: F, t1903: F, t22386: F, t22433: F, t23042: F, t4076: F, t47568: F, t47570: F, t49474: F, t49477: F, t49480: F, t5715: F, t5774: F, t6895: F, t74757: F, t74763: F, t74770: F, t74782: F, t9657: F, t6919: F, t5741: F, t74892: F, t22315: F, t48084: F, t22858: F, t47372: F, t686: F, t72: F, t1432: F, t22964: F, t14239: F, t22332: F, t10023: F, t22863: F, t1398: F, t14193: F, t1437: F, t22321: F, t47961: F, t47964: F, t5659: F, t5735: F, t74893: F, t74901: F, t74908: F, t820: F, t85638: F, t86054: F, t14141: F, t23037: F, t10049: F, t22954: F, t4118: F, t47967: F, t47971: F, t47979: F, t47981: F, t47985: F, t74935: F, t74943: F, t74945: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t86203, t86205, t86208, t86212, t86220) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3279::<F>(t22953, t2661, t3992, t543, t550, t22857, t46609, t9994, t4003, t9934, t221, t22809, t3978, t3979);
        let (t86222, t86226, t86234, t86236) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3280::<F>(t22815, t3989, t221, t22813, t3978, t46716, t1883, t22020, t2661, t3992, t22877, t46691);
        let t86249 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3281::<F>(t22822, t3989, t2661, t3992, t543, t86205, t1353, t1410, t1414, t221, t22852, t49071, t49093, t74638, t74641, t74656, t74660, t74664, t828, t85442, t86203, t86208, t86212, t86220, t86222, t86226, t86234, t86236);
        let (t86256, t86260, t86264, t86274) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3282::<F>(t221, t22912, t4018, t4019, t2661, t3992, t6869, t73920, t1883, t22245, t22841, t74026, t9934);
        let t86276 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3283::<F>(t47337, t49087, t49090, t49103, t49105, t49122, t49125, t49127, t74677, t74682, t74698, t74711, t74714, t74717, t86256, t86260, t86264, t86274);
        let t86280 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3284::<F>(t85562, t85623, t85680, t85709, t85738, t85780, t85830, t85854, t85871, t86086, t86106, t86136, t86162, t86198, t86249, t86276);
        let t86291 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3285::<F>(t14100, t22399, t1904, t213, t22390, t225, t47504, t47512, t47886, t47899, t47904, t561, t5728, t73666, t73671, t73673, t73676, t73705, t73707, t74802, t85509, t86280);
        let t86308 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3286::<F>(t5722, t74835, t1357, t23043, t689, t47561, t47907, t47920, t47932, t47938, t47942, t47945, t47948, t47953, t49468, t73712, t74733, t74744);
        let t86340 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3287::<F>(t1364, t22965, t786, t1357, t22975, t689, t5599, t6896, t1424, t1444, t1903, t22386, t22433, t23042, t4076, t47568, t47570, t49474, t49477, t49480, t5715, t5774, t6895, t74757, t74763, t74770, t74782, t9657);
        let (t86346, t86350, t86354, t86358, t86374) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3288::<F>(t5599, t689, t6919, t5741, t74892, t22315, t48084, t22858, t47372, t686, t72, t1432, t22964);
        let t86387 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3289::<F>(t14239, t22332, t10023, t22863, t686, t72, t1398, t14193, t1437, t22321, t47961, t47964, t5659, t5735, t74893, t74901, t74908, t820, t85638, t86054, t86374);
        let t86405 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3290::<F>(t14141, t23037, t686, t72, t10049, t22863, t22954, t4118, t47967, t47971, t47979, t47981, t47985, t74935, t74943, t74945, t820);
    (t86280, t86291, t86308, t86340, t86346, t86350, t86354, t86358, t86387, t86405)
}

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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta971(t22953: f64, t2661: f64, t3992: f64, t543: f64, t550: f64, t22857: f64, t46609: f64, t9994: f64, t4003: f64, t9934: f64, t221: f64, t22809: f64, t3978: f64, t3979: f64, t22815: f64, t3989: f64, t22813: f64, t46716: f64, t1883: f64, t22020: f64, t22877: f64, t46691: f64, t22822: f64, t1353: f64, t1410: f64, t1414: f64, t22852: f64, t49071: f64, t49093: f64, t74638: f64, t74641: f64, t74656: f64, t74660: f64, t74664: f64, t828: f64, t85442: f64, t22912: f64, t4018: f64, t4019: f64, t6869: f64, t73920: f64, t22245: f64, t22841: f64, t74026: f64, t47337: f64, t49087: f64, t49090: f64, t49103: f64, t49105: f64, t49122: f64, t49125: f64, t49127: f64, t74677: f64, t74682: f64, t74698: f64, t74711: f64, t74714: f64, t74717: f64, t85562: f64, t85623: f64, t85680: f64, t85709: f64, t85738: f64, t85780: f64, t85830: f64, t85854: f64, t85871: f64, t86086: f64, t86106: f64, t86136: f64, t86162: f64, t86198: f64, t14100: f64, t22399: f64, t1904: f64, t213: f64, t22390: f64, t225: f64, t47504: f64, t47512: f64, t47886: f64, t47899: f64, t47904: f64, t561: f64, t5728: f64, t73666: f64, t73671: f64, t73673: f64, t73676: f64, t73705: f64, t73707: f64, t74802: f64, t85509: f64, t5722: f64, t74835: f64, t1357: f64, t23043: f64, t689: f64, t47561: f64, t47907: f64, t47920: f64, t47932: f64, t47938: f64, t47942: f64, t47945: f64, t47948: f64, t47953: f64, t49468: f64, t73712: f64, t74733: f64, t74744: f64, t1364: f64, t22965: f64, t786: f64, t22975: f64, t5599: f64, t6896: f64, t1424: f64, t1444: f64, t1903: f64, t22386: f64, t22433: f64, t23042: f64, t4076: f64, t47568: f64, t47570: f64, t49474: f64, t49477: f64, t49480: f64, t5715: f64, t5774: f64, t6895: f64, t74757: f64, t74763: f64, t74770: f64, t74782: f64, t9657: f64, t6919: f64, t5741: f64, t74892: f64, t22315: f64, t48084: f64, t22858: f64, t47372: f64, t686: f64, t72: f64, t1432: f64, t22964: f64, t14239: f64, t22332: f64, t10023: f64, t22863: f64, t1398: f64, t14193: f64, t1437: f64, t22321: f64, t47961: f64, t47964: f64, t5659: f64, t5735: f64, t74893: f64, t74901: f64, t74908: f64, t820: f64, t85638: f64, t86054: f64, t14141: f64, t23037: f64, t10049: f64, t22954: f64, t4118: f64, t47967: f64, t47971: f64, t47979: f64, t47981: f64, t47985: f64, t74935: f64, t74943: f64, t74945: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86203, t86205, t86208, t86212, t86220) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3279(t22953, t2661, t3992, t543, t550, t22857, t46609, t9994, t4003, t9934, t221, t22809, t3978, t3979);
        let (t86222, t86226, t86234, t86236) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3280(t22815, t3989, t221, t22813, t3978, t46716, t1883, t22020, t2661, t3992, t22877, t46691);
        let t86249 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3281(t22822, t3989, t2661, t3992, t543, t86205, t1353, t1410, t1414, t221, t22852, t49071, t49093, t74638, t74641, t74656, t74660, t74664, t828, t85442, t86203, t86208, t86212, t86220, t86222, t86226, t86234, t86236);
        let (t86256, t86260, t86264, t86274) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3282(t221, t22912, t4018, t4019, t2661, t3992, t6869, t73920, t1883, t22245, t22841, t74026, t9934);
        let t86276 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3283(t47337, t49087, t49090, t49103, t49105, t49122, t49125, t49127, t74677, t74682, t74698, t74711, t74714, t74717, t86256, t86260, t86264, t86274);
        let t86280 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3284(t85562, t85623, t85680, t85709, t85738, t85780, t85830, t85854, t85871, t86086, t86106, t86136, t86162, t86198, t86249, t86276);
        let t86291 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3285(t14100, t22399, t1904, t213, t22390, t225, t47504, t47512, t47886, t47899, t47904, t561, t5728, t73666, t73671, t73673, t73676, t73705, t73707, t74802, t85509, t86280);
        let t86308 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3286(t5722, t74835, t1357, t23043, t689, t47561, t47907, t47920, t47932, t47938, t47942, t47945, t47948, t47953, t49468, t73712, t74733, t74744);
        let t86340 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3287(t1364, t22965, t786, t1357, t22975, t689, t5599, t6896, t1424, t1444, t1903, t22386, t22433, t23042, t4076, t47568, t47570, t49474, t49477, t49480, t5715, t5774, t6895, t74757, t74763, t74770, t74782, t9657);
        let (t86346, t86350, t86354, t86358, t86374) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3288(t5599, t689, t6919, t5741, t74892, t22315, t48084, t22858, t47372, t686, t72, t1432, t22964);
        let t86387 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3289(t14239, t22332, t10023, t22863, t686, t72, t1398, t14193, t1437, t22321, t47961, t47964, t5659, t5735, t74893, t74901, t74908, t820, t85638, t86054, t86374);
        let t86405 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3290(t14141, t23037, t686, t72, t10049, t22863, t22954, t4118, t47967, t47971, t47979, t47981, t47985, t74935, t74943, t74945, t820);
    (t86280, t86291, t86308, t86340, t86346, t86350, t86354, t86358, t86387, t86405)
}

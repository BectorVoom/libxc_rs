//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta418 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1491;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1492;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1493;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1494;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1495;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1496;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1497;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1498;
use chunk8::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1499;
use chunk9::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1500;
use chunk10::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1501;
use chunk11::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1502;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta418<F: Float>(t6017: F, t72: F, t686: F, t2798: F, t5978: F, t14568: F, t4500: F, t18699: F, t231: F, t2783: F, t2782: F, t18677: F, t18681: F, t2723: F, t4503: F, t10916: F, t14577: F, t14581: F, t14590: F, t14596: F, t14603: F, t14608: F, t14948: F, t6041: F, t874: F, t10661: F, t10923: F, t10925: F, t10939: F, t10948: F, t10964: F, t10966: F, t10969: F, t10971: F, t14546: F, t14951: F, t14972: F, t1559: F, t18525: F, t4366: F, t4504: F, t6022: F, t820: F, t18687: F, t18722: F, t868: F, t10503: F, t10507: F, t10511: F, t10984: F, t14998: F, t15004: F, t15006: F, t15010: F, t15015: F, t18324: F, t18658: F, t18663: F, t213: F, t257: F, t865: F, t6071: F, t2465: F, t6048: F, t10995: F, t10987: F, t11000: F, t11004: F, t11013: F, t11017: F, t11019: F, t11030: F, t15018: F, t15047: F, t15050: F, t887: F, t6072: F, t779: F, t689: F, t1580: F, t4321: F, t6042: F, t786: F, t789: F, t6049: F, t14987: F, t4481: F, t11040: F, t15011: F, t15062: F, t15063: F, t2765: F, t4474: F, t4487: F, t4534: F, t18322: F, t10563: F, t10566: F, t14324: F, t14343: F, t14345: F, t14372: F, t18392: F, t18535: F, t18536: F, t18537: F, t18538: F, t18541: F, t18543: F, t18546: F, t18548: F, t18549: F, t18552: F, t198: F, t207: F, t2403: F, t4343: F, t4546: F, t765: F, t892: F, t9394: F, t6075: F, t262: F, t5962: F, t10568: F, t10577: F, t10582: F, t10584: F, t10586: F, t14353: F, t14433: F, t1544: F, t18557: F, t18558: F, t18561: F, t18564: F, t18565: F, t18567: F, t2404: F, t4541: F, t775: F, t9514: F, t9517: F, t9521: F, t2411: F, t11064: F, t6079: F, t890: F, t10592: F, t10596: F, t10604: F, t10611: F, t11088: F, t14618: F, t18571: F, t18572: F, t18573: F, t18574: F, t18578: F, t18579: F, t18581: F, t18582: F, t1940: F, t4433: F, t4556: F, t5966: F, t9524: F, t9542: F, t18309: F, t1587: F, t2: F, t580: F, t11506: F, t6189: F, t11509: F, t972: F, t981: F, t11144: F, t5819: F, t606: F, t11142: F, t128: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t18727, t18731, t18733, t18739, t18742) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1491::<F>(t6017, t72, t686, t2798, t5978, t14568, t4500, t18699, t231, t2783, t2782, t18677);
        let t18754 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1492::<F>(t18742, t2782, t18681, t231, t2783, t18677, t2723, t4503, t10916, t14577, t14581, t14590, t14596, t14603, t14608, t14948, t18727, t18731, t18733, t18739);
        let t18782 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1493::<F>(t6041, t72, t686, t874, t10661, t10923, t10925, t10939, t10948, t10964, t10966, t10969, t10971, t14546, t14951, t14972, t1559, t18525, t18677, t18681, t18699, t4366, t4504, t6022, t820);
        let (t18784, t18785, t18791) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1494::<F>(t18687, t18722, t18754, t18782, t868, t10503, t10507, t10511, t10984, t14998, t15004, t15006, t15010, t15015, t18324, t18658, t18663, t213, t257, t865);
        let (t18797, t18805, t18810) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1495::<F>(t6071, t72, t686, t2465, t213, t6041, t6048, t10995, t10987, t11000, t11004, t11013, t11017, t11019, t11030, t15018, t15047, t15050, t887);
        let t18836 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1496::<F>(t6072, t779, t689, t1580, t4321, t6042, t786, t789, t6049, t14987, t4481, t11040, t15011, t15062, t15063, t2765, t4474, t4487, t4534);
        let (t18838, t18848) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1497::<F>(t18322, t18791, t18810, t18836, t10563, t10566, t14324, t14343, t14345, t14372, t18392, t18535, t18536, t18537, t18538, t18541, t18543, t18546, t18548, t18549, t18552, t198, t207, t2403, t4343, t4546, t765, t892, t9394);
        let t18864 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1498::<F>(t6075, t892, t262, t5962, t10568, t10577, t10582, t10584, t10586, t14353, t14433, t1544, t18557, t18558, t18561, t18564, t18565, t18567, t2403, t2404, t4541, t775, t9514, t9517, t9521);
        let (t18865, t18871, t18875) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1499::<F>(t2411, t6075, t11064, t6079, t1544, t890);
        let t18882 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1500::<F>(t10592, t10596, t10604, t10611, t11088, t14618, t18571, t18572, t18573, t18574, t18578, t18579, t18581, t18582, t18865, t18871, t18875, t1940, t198, t2403, t4433, t4541, t4546, t4556, t5966, t890, t9524, t9542);
        let (t18884, t18892, t18902) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1501::<F>(t18309, t18848, t18864, t18882, t1587, t2, t580, t11506, t6189, t11509, t972, t981);
        let (t18904, t18906) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1502::<F>(t11144, t5819, t606, t11142, t128);
    (t18784, t18785, t18797, t18805, t18838, t18875, t18884, t18892, t18902, t18904, t18906)
}

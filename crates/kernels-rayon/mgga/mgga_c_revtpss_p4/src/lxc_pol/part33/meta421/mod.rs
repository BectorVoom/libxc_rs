//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta421 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1498;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1499;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1500;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1501;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1502;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1503;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1504;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1505;
use chunk8::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1506;
use chunk9::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1507;
use chunk10::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1508;
use chunk11::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1509;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta421(t6017: f64, t72: f64, t686: f64, t2798: f64, t5978: f64, t14568: f64, t4500: f64, t18699: f64, t231: f64, t2783: f64, t2782: f64, t18677: f64, t18681: f64, t2723: f64, t4503: f64, t10916: f64, t14577: f64, t14581: f64, t14590: f64, t14596: f64, t14603: f64, t14608: f64, t14948: f64, t6041: f64, t874: f64, t10661: f64, t10923: f64, t10925: f64, t10939: f64, t10948: f64, t10964: f64, t10966: f64, t10969: f64, t10971: f64, t14546: f64, t14951: f64, t14972: f64, t1559: f64, t18525: f64, t4366: f64, t4504: f64, t6022: f64, t820: f64, t18687: f64, t18722: f64, t868: f64, t10503: f64, t10507: f64, t10511: f64, t10984: f64, t14998: f64, t15004: f64, t15006: f64, t15010: f64, t15015: f64, t18324: f64, t18658: f64, t18663: f64, t213: f64, t257: f64, t865: f64, t6071: f64, t2465: f64, t6048: f64, t10995: f64, t10987: f64, t11000: f64, t11004: f64, t11013: f64, t11017: f64, t11019: f64, t11030: f64, t15018: f64, t15047: f64, t15050: f64, t887: f64, t6072: f64, t779: f64, t689: f64, t1580: f64, t4321: f64, t6042: f64, t786: f64, t789: f64, t6049: f64, t14987: f64, t4481: f64, t11040: f64, t15011: f64, t15062: f64, t15063: f64, t2765: f64, t4474: f64, t4487: f64, t4534: f64, t18322: f64, t10563: f64, t10566: f64, t14324: f64, t14343: f64, t14345: f64, t14372: f64, t18392: f64, t18535: f64, t18536: f64, t18537: f64, t18538: f64, t18541: f64, t18543: f64, t18546: f64, t18548: f64, t18549: f64, t18552: f64, t198: f64, t207: f64, t2403: f64, t4343: f64, t4546: f64, t765: f64, t892: f64, t9394: f64, t6075: f64, t262: f64, t5962: f64, t10568: f64, t10577: f64, t10582: f64, t10584: f64, t10586: f64, t14353: f64, t14433: f64, t1544: f64, t18557: f64, t18558: f64, t18561: f64, t18564: f64, t18565: f64, t18567: f64, t2404: f64, t4541: f64, t775: f64, t9514: f64, t9517: f64, t9521: f64, t2411: f64, t11064: f64, t6079: f64, t890: f64, t10592: f64, t10596: f64, t10604: f64, t10611: f64, t11088: f64, t14618: f64, t18571: f64, t18572: f64, t18573: f64, t18574: f64, t18578: f64, t18579: f64, t18581: f64, t18582: f64, t1940: f64, t4433: f64, t4556: f64, t5966: f64, t9524: f64, t9542: f64, t18309: f64, t1587: f64, t2: f64, t580: f64, t11506: f64, t6189: f64, t11509: f64, t972: f64, t981: f64, t11144: f64, t5819: f64, t606: f64, t11142: f64, t128: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18727, t18731, t18733, t18739, t18742) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1498(t6017, t72, t686, t2798, t5978, t14568, t4500, t18699, t231, t2783, t2782, t18677);
        let t18754 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1499(t18742, t2782, t18681, t231, t2783, t18677, t2723, t4503, t10916, t14577, t14581, t14590, t14596, t14603, t14608, t14948, t18727, t18731, t18733, t18739);
        let t18782 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1500(t6041, t72, t686, t874, t10661, t10923, t10925, t10939, t10948, t10964, t10966, t10969, t10971, t14546, t14951, t14972, t1559, t18525, t18677, t18681, t18699, t4366, t4504, t6022, t820);
        let (t18784, t18785, t18791) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1501(t18687, t18722, t18754, t18782, t868, t10503, t10507, t10511, t10984, t14998, t15004, t15006, t15010, t15015, t18324, t18658, t18663, t213, t257, t865);
        let (t18797, t18805, t18810) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1502(t6071, t72, t686, t2465, t213, t6041, t6048, t10995, t10987, t11000, t11004, t11013, t11017, t11019, t11030, t15018, t15047, t15050, t887);
        let t18836 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1503(t6072, t779, t689, t1580, t4321, t6042, t786, t789, t6049, t14987, t4481, t11040, t15011, t15062, t15063, t2765, t4474, t4487, t4534);
        let (t18838, t18848) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1504(t18322, t18791, t18810, t18836, t10563, t10566, t14324, t14343, t14345, t14372, t18392, t18535, t18536, t18537, t18538, t18541, t18543, t18546, t18548, t18549, t18552, t198, t207, t2403, t4343, t4546, t765, t892, t9394);
        let t18864 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1505(t6075, t892, t262, t5962, t10568, t10577, t10582, t10584, t10586, t14353, t14433, t1544, t18557, t18558, t18561, t18564, t18565, t18567, t2403, t2404, t4541, t775, t9514, t9517, t9521);
        let (t18865, t18871, t18875) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1506(t2411, t6075, t11064, t6079, t1544, t890);
        let t18882 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1507(t10592, t10596, t10604, t10611, t11088, t14618, t18571, t18572, t18573, t18574, t18578, t18579, t18581, t18582, t18865, t18871, t18875, t1940, t198, t2403, t4433, t4541, t4546, t4556, t5966, t890, t9524, t9542);
        let (t18884, t18892, t18902) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1508(t18309, t18848, t18864, t18882, t1587, t2, t580, t11506, t6189, t11509, t972, t981);
        let (t18904, t18906) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1509(t11144, t5819, t606, t11142, t128);
    (t18784, t18785, t18797, t18805, t18838, t18875, t18884, t18892, t18902, t18904, t18906)
}

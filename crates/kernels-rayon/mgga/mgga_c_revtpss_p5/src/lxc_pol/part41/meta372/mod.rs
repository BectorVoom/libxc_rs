//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta372 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1213;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1214;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1215;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1216;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1217;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1218;
use chunk6::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1219;
use chunk7::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1220;
use chunk8::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1221;
use chunk9::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1222;
use chunk10::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1223;
use chunk11::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1224;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta372(t2723: f64, t4423: f64, t4364: f64, t4365: f64, t231: f64, t4343: f64, t2747: f64, t10779: f64, t14671: f64, t6035: f64, t10777: f64, t14676: f64, t18444: f64, t837: f64, t14894: f64, t14907: f64, t14925: f64, t14934: f64, t18527: f64, t18532: f64, t18618: f64, t18623: f64, t18629: f64, t2745: f64, t4362: f64, t825: f64, t18330: f64, t18343: f64, t18361: f64, t18405: f64, t18454: f64, t18489: f64, t18524: f64, t225: f64, t6048: f64, t886: f64, t11008: f64, t251: f64, t5977: f64, t1558: f64, t1568: f64, t10519: f64, t10539: f64, t14498: f64, t14506: f64, t14511: f64, t14512: f64, t14518: f64, t14522: f64, t14525: f64, t14533: f64, t14539: f64, t2815: f64, t4424: f64, t4494: f64, t4514: f64, t5978: f64, t820: f64, t233: f64, t6041: f64, t869: f64, t689: f64, t6016: f64, t822: f64, t6022: f64, t72: f64, t686: f64, t10530: f64, t10645: f64, t10647: f64, t10651: f64, t14558: f64, t14564: f64, t14570: f64, t18616: f64, t213: f64, t234: f64, t4504: f64, t4526: f64, t6017: f64, t879: f64, t2798: f64, t14568: f64, t4500: f64, t2783: f64, t2782: f64, t4503: f64, t10916: f64, t14577: f64, t14581: f64, t14590: f64, t14596: f64, t14603: f64, t14608: f64, t14948: f64, t874: f64, t10661: f64, t10923: f64, t10925: f64, t10939: f64, t10948: f64, t10964: f64, t10966: f64, t10969: f64, t10971: f64, t14546: f64, t14951: f64, t14972: f64, t1559: f64, t18525: f64, t4366: f64, t868: f64, t10503: f64, t10507: f64, t10511: f64, t10984: f64, t14998: f64, t15004: f64, t15006: f64, t15010: f64, t15015: f64, t18324: f64, t257: f64, t865: f64, t6071: f64, t2465: f64, t10995: f64, t10987: f64, t11000: f64, t11004: f64, t11013: f64, t11017: f64, t11019: f64, t11030: f64, t15018: f64, t15047: f64, t15050: f64, t887: f64, t6072: f64, t779: f64, t1580: f64, t4321: f64, t6042: f64, t786: f64, t789: f64, t6049: f64, t14987: f64, t4481: f64, t11040: f64, t15011: f64, t15062: f64, t15063: f64, t2765: f64, t4474: f64, t4487: f64, t4534: f64, t18322: f64, t10563: f64, t10566: f64, t14324: f64, t14343: f64, t14345: f64, t14372: f64, t18392: f64, t18535: f64, t18536: f64, t18537: f64, t18538: f64, t18541: f64, t18543: f64, t18546: f64, t18548: f64, t18549: f64, t18552: f64, t198: f64, t207: f64, t2403: f64, t4546: f64, t765: f64, t892: f64, t9394: f64) -> f64 {
        let (t18632, t18634, t18639, t18644, t18647) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1213(t2723, t4423, t4364, t4365, t231, t4343, t2747, t10779, t14671, t6035, t10777, t14676);
        let t18654 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1214(t18444, t4364, t837, t14894, t14907, t14925, t14934, t18527, t18532, t18618, t18623, t18629, t18634, t18639, t18644, t18647, t2745, t4362, t825);
        let (t18657, t18658, t18663) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1215(t18330, t18343, t18361, t18405, t18454, t18489, t18524, t18654, t225, t6048, t886, t11008);
        let (t18677, t18681, t18687) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1216(t251, t5977, t1558, t1568, t10519, t10539, t14498, t14506, t14511, t14512, t14518, t14522, t14525, t14533, t14539, t2815, t4424, t4494, t4514, t5978, t820, t837);
        let (t18699, t18722) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1217(t233, t6041, t869, t689, t251, t6016, t822, t6022, t72, t686, t10530, t10645, t10647, t10651, t14558, t14564, t14570, t18616, t18632, t18657, t213, t234, t2815, t4424, t4494, t4504, t4514, t4526, t6017, t820, t837, t879);
        let (t18727, t18731, t18733, t18739, t18742) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1218(t6017, t72, t686, t2798, t5978, t14568, t4500, t18699, t231, t2783, t2782, t18677);
        let t18754 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1219(t18742, t2782, t18681, t231, t2783, t18677, t2723, t4503, t10916, t14577, t14581, t14590, t14596, t14603, t14608, t14948, t18727, t18731, t18733, t18739);
        let t18782 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1220(t6041, t72, t686, t874, t10661, t10923, t10925, t10939, t10948, t10964, t10966, t10969, t10971, t14546, t14951, t14972, t1559, t18525, t18677, t18681, t18699, t4366, t4504, t6022, t820);
        let t18791 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1221(t18687, t18722, t18754, t18782, t868, t10503, t10507, t10511, t10984, t14998, t15004, t15006, t15010, t15015, t18324, t18658, t18663, t213, t257, t865);
        let t18810 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1222(t6071, t72, t686, t2465, t213, t6041, t6048, t10995, t10987, t11000, t11004, t11013, t11017, t11019, t11030, t15018, t15047, t15050, t887);
        let t18836 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1223(t6072, t779, t689, t1580, t4321, t6042, t786, t789, t6049, t14987, t4481, t11040, t15011, t15062, t15063, t2765, t4474, t4487, t4534);
        let t18848 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1224(t18322, t18791, t18810, t18836, t10563, t10566, t14324, t14343, t14345, t14372, t18392, t18535, t18536, t18537, t18538, t18541, t18543, t18546, t18548, t18549, t18552, t198, t207, t2403, t4343, t4546, t765, t892, t9394);
    t18848
}

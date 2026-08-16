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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta372<F: Float>(t2723: F, t4423: F, t4364: F, t4365: F, t231: F, t4343: F, t2747: F, t10779: F, t14671: F, t6035: F, t10777: F, t14676: F, t18444: F, t837: F, t14894: F, t14907: F, t14925: F, t14934: F, t18527: F, t18532: F, t18618: F, t18623: F, t18629: F, t2745: F, t4362: F, t825: F, t18330: F, t18343: F, t18361: F, t18405: F, t18454: F, t18489: F, t18524: F, t225: F, t6048: F, t886: F, t11008: F, t251: F, t5977: F, t1558: F, t1568: F, t10519: F, t10539: F, t14498: F, t14506: F, t14511: F, t14512: F, t14518: F, t14522: F, t14525: F, t14533: F, t14539: F, t2815: F, t4424: F, t4494: F, t4514: F, t5978: F, t820: F, t233: F, t6041: F, t869: F, t689: F, t6016: F, t822: F, t6022: F, t72: F, t686: F, t10530: F, t10645: F, t10647: F, t10651: F, t14558: F, t14564: F, t14570: F, t18616: F, t213: F, t234: F, t4504: F, t4526: F, t6017: F, t879: F, t2798: F, t14568: F, t4500: F, t2783: F, t2782: F, t4503: F, t10916: F, t14577: F, t14581: F, t14590: F, t14596: F, t14603: F, t14608: F, t14948: F, t874: F, t10661: F, t10923: F, t10925: F, t10939: F, t10948: F, t10964: F, t10966: F, t10969: F, t10971: F, t14546: F, t14951: F, t14972: F, t1559: F, t18525: F, t4366: F, t868: F, t10503: F, t10507: F, t10511: F, t10984: F, t14998: F, t15004: F, t15006: F, t15010: F, t15015: F, t18324: F, t257: F, t865: F, t6071: F, t2465: F, t10995: F, t10987: F, t11000: F, t11004: F, t11013: F, t11017: F, t11019: F, t11030: F, t15018: F, t15047: F, t15050: F, t887: F, t6072: F, t779: F, t1580: F, t4321: F, t6042: F, t786: F, t789: F, t6049: F, t14987: F, t4481: F, t11040: F, t15011: F, t15062: F, t15063: F, t2765: F, t4474: F, t4487: F, t4534: F, t18322: F, t10563: F, t10566: F, t14324: F, t14343: F, t14345: F, t14372: F, t18392: F, t18535: F, t18536: F, t18537: F, t18538: F, t18541: F, t18543: F, t18546: F, t18548: F, t18549: F, t18552: F, t198: F, t207: F, t2403: F, t4546: F, t765: F, t892: F, t9394: F) -> F {
        let (t18632, t18634, t18639, t18644, t18647) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1213::<F>(t2723, t4423, t4364, t4365, t231, t4343, t2747, t10779, t14671, t6035, t10777, t14676);
        let t18654 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1214::<F>(t18444, t4364, t837, t14894, t14907, t14925, t14934, t18527, t18532, t18618, t18623, t18629, t18634, t18639, t18644, t18647, t2745, t4362, t825);
        let (t18657, t18658, t18663) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1215::<F>(t18330, t18343, t18361, t18405, t18454, t18489, t18524, t18654, t225, t6048, t886, t11008);
        let (t18677, t18681, t18687) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1216::<F>(t251, t5977, t1558, t1568, t10519, t10539, t14498, t14506, t14511, t14512, t14518, t14522, t14525, t14533, t14539, t2815, t4424, t4494, t4514, t5978, t820, t837);
        let (t18699, t18722) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1217::<F>(t233, t6041, t869, t689, t251, t6016, t822, t6022, t72, t686, t10530, t10645, t10647, t10651, t14558, t14564, t14570, t18616, t18632, t18657, t213, t234, t2815, t4424, t4494, t4504, t4514, t4526, t6017, t820, t837, t879);
        let (t18727, t18731, t18733, t18739, t18742) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1218::<F>(t6017, t72, t686, t2798, t5978, t14568, t4500, t18699, t231, t2783, t2782, t18677);
        let t18754 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1219::<F>(t18742, t2782, t18681, t231, t2783, t18677, t2723, t4503, t10916, t14577, t14581, t14590, t14596, t14603, t14608, t14948, t18727, t18731, t18733, t18739);
        let t18782 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1220::<F>(t6041, t72, t686, t874, t10661, t10923, t10925, t10939, t10948, t10964, t10966, t10969, t10971, t14546, t14951, t14972, t1559, t18525, t18677, t18681, t18699, t4366, t4504, t6022, t820);
        let t18791 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1221::<F>(t18687, t18722, t18754, t18782, t868, t10503, t10507, t10511, t10984, t14998, t15004, t15006, t15010, t15015, t18324, t18658, t18663, t213, t257, t865);
        let t18810 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1222::<F>(t6071, t72, t686, t2465, t213, t6041, t6048, t10995, t10987, t11000, t11004, t11013, t11017, t11019, t11030, t15018, t15047, t15050, t887);
        let t18836 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1223::<F>(t6072, t779, t689, t1580, t4321, t6042, t786, t789, t6049, t14987, t4481, t11040, t15011, t15062, t15063, t2765, t4474, t4487, t4534);
        let t18848 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1224::<F>(t18322, t18791, t18810, t18836, t10563, t10566, t14324, t14343, t14345, t14372, t18392, t18535, t18536, t18537, t18538, t18541, t18543, t18546, t18548, t18549, t18552, t198, t207, t2403, t4343, t4546, t765, t892, t9394);
    t18848
}

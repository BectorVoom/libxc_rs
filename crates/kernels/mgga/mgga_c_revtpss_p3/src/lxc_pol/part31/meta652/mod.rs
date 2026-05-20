//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta652 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2164;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2165;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2166;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2167;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2168;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2169;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2170;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2171;
use chunk8::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2172;
use chunk9::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2173;
use chunk10::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2174;
use chunk11::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2175;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta652<F: Float>(t4746: F, t7143: F, t1646: F, t1695: F, t100691: F, t100760: F, t1043: F, t107201: F, t1089: F, t1096: F, t1652: F, t1668: F, t19385: F, t19425: F, t1985: F, t225: F, t25629: F, t27422: F, t27423: F, t27568: F, t27587: F, t27599: F, t27679: F, t29747: F, t29759: F, t29887: F, t342: F, t385: F, t4941: F, t7102: F, t7140: F, t7151: F, t7160: F, t7837: F, t93436: F, t93438: F, t93921: F, t999: F, t99934: F, t106655: F, t994: F, t106719: F, t107226: F, t1651: F, t1696: F, t19477: F, t25591: F, t25640: F, t25671: F, t25681: F, t27419: F, t27557: F, t27684: F, t29727: F, t29807: F, t29826: F, t29852: F, t3318: F, t4772: F, t5015: F, t6299: F, t6305: F, t7144: F, t7145: F, t7147: F, t7159: F, t7167: F, t7168: F, t7817: F, t93490: F, t93983: F, t93984: F, t94016: F, t988: F, t99666: F, t29833: F, t3056: F, t100723: F, t19380: F, t19520: F, t1976: F, t25464: F, t25611: F, t25658: F, t27412: F, t27550: F, t27609: F, t27642: F, t27669: F, t29731: F, t29751: F, t29865: F, t29871: F, t3304: F, t4764: F, t6393: F, t7153: F, t7829: F, t93498: F, t93516: F, t93994: F, t94053: F, t100705: F, t19483: F, t20215: F, t25476: F, t25695: F, t25699: F, t27579: F, t27635: F, t27640: F, t27661: F, t27692: F, t29844: F, t29876: F, t29883: F, t4773: F, t6251: F, t7810: F, t94063: F, t94064: F, t94095: F, t99969: F, t1000: F, t100596: F, t107268: F, t19491: F, t19548: F, t27680: F, t27688: F, t29809: F, t29848: F, t7828: F, t93890: F, t93893: F, t94080: F, t94081: F, t989: F, t99684: F, t99947: F, t1647: F, t1078: F, t1982: F, t3140: F, t6343: F, t100702: F, t1097: F, t19381: F, t1986: F, t20112: F, t27415: F, t27433: F, t27445: F, t27621: F, t27627: F, t29866: F, t6235: F, t6244: F, t6259: F, t7135: F, t7137: F, t7170: F, t94122: F, t99675: F, t99940: F, t100708: F, t19396: F, t1978: F, t19856: F, t25473: F, t25634: F, t27437: F, t27543: F, t27545: F, t27604: F, t27639: F, t27643: F, t27647: F, t27665: F, t27668: F, t27670: F, t27699: F, t29752: F, t4743: F, t4866: F, t5016: F, t6351: F, t7812: F, t7825: F, t99909: F, t99915: F, t100494: F, t19400: F, t19415: F, t19502: F, t25461: F, t25605: F, t25651: F, t27631: F, t27653: F, t29818: F, t29835: F, t29872: F, t6234: F, t7174: F, t7822: F, t93928: F, t94085: F, t29894: F, t3336: F, t100802: F, t100806: F, t106684: F, t106738: F, t106786: F, t106834: F, t107206: F, t107257: F, t107305: F, t107354: F, t1100: F, t1102: F, t1699: F, t198: F, t20230: F, t25709: F, t25713: F, t27712: F, t27717: F, t336: F, t5019: F, t5023: F, t6396: F, t6400: F, t7181: F, t94142: F, t94149: F, t27375: F, t63185: F, t11064: F, t1544: F, t27384: F, t105923: F, t106481: F, t106516: F, t106610: F, t1583: F, t18392: F, t18498: F, t1940: F, t1963: F, t207: F, t2403: F, t25206: F, t25440: F, t25445: F, t27158: F, t29598: F, t4343: F, t4433: F, t4541: F, t5962: F, t6075: F, t7087: F, t7091: F, t77408: F, t7783: F, t890: F, t892: F, t98722: F, t99555: F, t106554: F, t106561: F, t106565: F, t106625: F, t18435: F, t18838: F, t18875: F, t27364: F, t27368: F, t29705: F, t29907: F, t4537: F, t50080: F, t5966: F, t6079: F, t77425: F, t77441: F, t775: F, t92742: F, t93404: F, t30: F, t265: F, t393: F, t106638: F, t1469: F, t18281: F, t1996: F, t27755: F, t29931: F, t4186: F, t45: F, t5825: F, t606: F, t7194: F, t7856: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
        let t107405 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2164::<F>(t4746, t7143, t1646, t1695, t100691, t100760, t1043, t107201, t1089, t1096, t1652, t1668, t19385, t19425, t1985, t225, t25629, t27422, t27423, t27568, t27587, t27599, t27679, t29747, t29759, t29887, t342, t385, t4941, t7102, t7140, t7151, t7160, t7837, t93436, t93438, t93921, t999, t99934);
        let t107457 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2165::<F>(t106655, t994, t106719, t107226, t1089, t1096, t1651, t1696, t19477, t25591, t25640, t25671, t25681, t27419, t27557, t27679, t27684, t29727, t29759, t29807, t29826, t29852, t3318, t4772, t5015, t6299, t6305, t7144, t7145, t7147, t7159, t7160, t7167, t7168, t7817, t93490, t93983, t93984, t94016, t988, t99666);
        let t107509 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2166::<F>(t29833, t3056, t7143, t100723, t1043, t1089, t19380, t19520, t1976, t25464, t25611, t25658, t25671, t27412, t27550, t27609, t27642, t27669, t29727, t29731, t29751, t29807, t29865, t29871, t3304, t4764, t6305, t6393, t7144, t7145, t7151, t7153, t7160, t7829, t93436, t93498, t93516, t93994, t94053, t988, t999);
        let t107557 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2167::<F>(t1646, t1651, t100705, t107226, t1096, t1695, t19483, t1985, t20215, t25476, t25591, t25695, t25699, t27423, t27568, t27579, t27635, t27640, t27642, t27661, t27692, t29844, t29876, t29883, t4773, t5015, t6251, t7102, t7145, t7151, t7159, t7160, t7810, t93921, t94063, t94064, t94095, t988, t999, t99969);
        let t107603 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2168::<F>(t29807, t994, t1000, t100596, t107226, t107268, t1096, t1652, t19491, t19548, t25464, t25699, t27419, t27568, t27661, t27680, t27688, t29727, t29751, t29809, t29848, t29883, t4764, t5015, t7145, t7151, t7159, t7160, t7828, t93490, t93890, t93893, t94080, t94081, t989, t99684, t999, t99947);
        let t107649 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2169::<F>(t1647, t7810, t1078, t1982, t3140, t6343, t100702, t1043, t1089, t1097, t1651, t1652, t1696, t19381, t1986, t20112, t25591, t25695, t25699, t27415, t27422, t27433, t27445, t27621, t27627, t27661, t29747, t29866, t29871, t6235, t6244, t6259, t7102, t7135, t7137, t7145, t7170, t94122, t99675, t999, t99940);
        let t107691 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2170::<F>(t100708, t1089, t1646, t1647, t1652, t19396, t1978, t19856, t25473, t25634, t27437, t27543, t27545, t27604, t27639, t27643, t27647, t27665, t27668, t27670, t27699, t29752, t29807, t4743, t4866, t5016, t6351, t7102, t7144, t7145, t7151, t7167, t7812, t7825, t999, t99909, t99915);
        let t107733 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2171::<F>(t100494, t1043, t107226, t1089, t19400, t19415, t19502, t25461, t25476, t25605, t25634, t25651, t27550, t27621, t27631, t27653, t29731, t29818, t29835, t29866, t29872, t29887, t4772, t4941, t6234, t6393, t7135, t7144, t7145, t7151, t7160, t7174, t7810, t7822, t93928, t94085, t99675, t999);
        let t107772 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2172::<F>(t29894, t3336, t100802, t100806, t106684, t106738, t106786, t106834, t107206, t107257, t107305, t107354, t107405, t107457, t107509, t107557, t107603, t107649, t107691, t107733, t1100, t1102, t1699, t198, t20230, t25709, t25713, t27712, t27717, t336, t5019, t5023, t6396, t6400, t7181, t94142, t94149);
        let t107820 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2173::<F>(t27375, t63185, t11064, t1544, t27384, t105923, t106481, t106516, t106610, t1583, t18392, t18498, t1940, t1963, t198, t207, t2403, t25206, t25440, t25445, t27158, t29598, t4343, t4433, t4541, t5962, t6075, t7087, t7091, t77408, t7783, t890, t892, t98722, t99555);
        let t107867 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2174::<F>(t106554, t106561, t106565, t106625, t1544, t18435, t18838, t18875, t1940, t1963, t2403, t25445, t27364, t27368, t27375, t29705, t29907, t4537, t4541, t50080, t5966, t6079, t7087, t7091, t77425, t77441, t775, t92742, t93404);
        let (t107868, t107881) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2175::<F>(t30, t265, t393, t107820, t107867, t107772, t106638, t1469, t18281, t1996, t27755, t29931, t4186, t45, t5825, t606, t7194, t7856, dens_threshold, rho0, zeta_threshold);
    (t107868, t107881)
}

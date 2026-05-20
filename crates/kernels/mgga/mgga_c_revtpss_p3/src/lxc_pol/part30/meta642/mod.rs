//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta642 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2237;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2238;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2239;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2240;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2241;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2242;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2243;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2244;
use chunk8::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2245;
use chunk9::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2246;
use chunk10::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2247;
use chunk11::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2248;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta642<F: Float>(t26866: F, t5436: F, t17225: F, t7624: F, t17381: F, t17456: F, t17552: F, t17674: F, t17679: F, t17684: F, t26852: F, t26867: F, t29097: F, t29100: F, t3631: F, t5270: F, t5299: F, t97149: F, t97218: F, t97250: F, t97261: F, t17361: F, t7618: F, t17289: F, t2138: F, t1238: F, t16729: F, t17461: F, t17536: F, t17662: F, t26880: F, t29047: F, t29054: F, t29086: F, t3663: F, t97174: F, t97179: F, t97220: F, t97222: F, t97239: F, t97247: F, t3666: F, t8184: F, t17307: F, t17451: F, t1285: F, t97173: F, t104646: F, t17735: F, t16715: F, t17502: F, t17541: F, t17584: F, t17635: F, t17696: F, t17739: F, t3674: F, t5279: F, t57549: F, t17617: F, t26870: F, t3682: F, t8172: F, t29020: F, t3704: F, t3678: F, t16733: F, t16738: F, t16742: F, t17515: F, t29048: F, t97267: F, t97269: F, t97272: F, t3655: F, t8185: F, t17628: F, t7607: F, t17280: F, t17651: F, t17800: F, t1791: F, t26827: F, t5320: F, t7613: F, t97279: F, t97281: F, t97283: F, t97288: F, t97296: F, t17445: F, t8177: F, t1256: F, t29074: F, t29069: F, t29089: F, t3685: F, t17332: F, t17405: F, t3650: F, t3689: F, t3694: F, t3701: F, t484: F, t104624: F, t104626: F, t104636: F, t104640: F, t104666: F, t104692: F, t104718: F, t104746: F, t104772: F, t104796: F, t104821: F, t104825: F, t104828: F, t104833: F, t104834: F, t104844: F, t104876: F, t1252: F, t12966: F, t17222: F, t17237: F, t17254: F, t17700: F, t17796: F, t1808: F, t29040: F, t3591: F, t3714: F, t5386: F, t5397: F, t7623: F, t97112: F, t97138: F, t97200: F, t26948: F, t97065: F, t104606: F, t1203: F, t1214: F, t1294: F, t18090: F, t2149: F, t2150: F, t2151: F, t26906: F, t26937: F, t26969: F, t29109: F, t29122: F, t29158: F, t29179: F, t29217: F, t29264: F, t29282: F, t3552: F, t3575: F, t3601: F, t3738: F, t3783: F, t473: F, t5236: F, t5457: F, t7602: F, t7636: F, t7651: F, t7652: F, t8192: F, t8197: F, t8208: F, t8213: F, t96927: F, t97050: F, t97066: F, t97308: F, t97363: F, t97377: F, t97453: F, t3596: F, t8190: F, t1248: F, t1287: F, t1769: F, t1774: F, t1775: F, t17992: F, t18103: F, t26884: F, t26922: F, t26949: F, t26976: F, t29175: F, t29178: F, t29204: F, t29220: F, t29278: F, t29304: F, t3576: F, t3588: F, t3769: F, t3790: F, t7632: F, t7637: F, t7643: F, t7659: F, t96910: F, t97419: F, t1794: F, t7627: F, t3153: F, t3555: F, t1215: F, t18030: F, t18070: F, t18073: F, t225: F, t26933: F, t26971: F, t29129: F, t29141: F, t29193: F, t29194: F, t29196: F, t29207: F, t29216: F, t3739: F, t460: F, t494: F, t5465: F, t7648: F, t8202: F, t96861: F, t96870: F, t96929: F, t96953: F, t97313: F, t97397: F, t1243: F, t73: F, t1032: F, t5412: F, t2148: F, t12657: F, t17968: F, t1829: F, t26889: F, t26895: F, t26963: F, t26999: F, t29118: F, t29159: F, t29167: F, t29271: F, t29275: F, t3584: F, t5237: F, t5458: F, t7635: F, t7654: F, t8198: F, t8201: F, t97422: F, t97425: F, t1276: F, t3140: F, t1828: F, t104529: F, t26913: F, t26936: F, t26959: F, t26979: F, t27005: F, t27008: F, t27011: F, t27025: F, t29136: F, t29166: F, t29195: F, t29233: F, t29301: F, t3721: F, t5246: F, t5429: F, t5464: F, t7662: F, t8205: F, t96986: F, t97304: F) -> (F, F, F, F, F, F, F) {
        let t104900 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2237::<F>(t26866, t5436, t17225, t7624, t17381, t17456, t17552, t17674, t17679, t17684, t26852, t26867, t29097, t29100, t3631, t5270, t5299, t97149, t97218, t97250, t97261);
        let t104921 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2238::<F>(t17361, t7618, t17289, t2138, t1238, t16729, t17461, t17536, t17662, t26880, t29047, t29054, t29086, t3663, t97174, t97179, t97220, t97222, t97239, t97247);
        let (t104943, t104951) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2239::<F>(t3666, t8184, t17307, t2138, t17451, t26867, t1285, t97173, t104646, t17735, t1238, t16715, t17502, t17541, t17584, t17635, t17696, t17739, t26880, t29047, t3674, t5279, t57549, t97250);
        let t104973 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2240::<F>(t17617, t26870, t3682, t8172, t29020, t3704, t29086, t3678, t16733, t16738, t16742, t17515, t29047, t29048, t97174, t97267, t97269, t97272);
        let t104992 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2241::<F>(t3655, t8185, t17628, t7607, t104943, t17280, t17651, t17800, t1791, t26827, t5320, t7613, t97174, t97279, t97281, t97283, t97288, t97296);
        let t105017 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2242::<F>(t17445, t7607, t3655, t8177, t1256, t29074, t29069, t29089, t3685, t17332, t17405, t2138, t3650, t3689, t3694, t3701, t484, t8184);
        let t105021 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2243::<F>(t104624, t104626, t104636, t104640, t104666, t104692, t104718, t104746, t104772, t104796, t104821, t104825, t104828, t104833, t104834, t104844, t104876, t104900, t104921, t104951, t104973, t104992, t105017, t1252, t12966, t17222, t17237, t17254, t17700, t17796, t1808, t26852, t26880, t29020, t29040, t3591, t3714, t5386, t5397, t7618, t7623, t7624, t97112, t97138, t97200);
        let t105057 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2244::<F>(t26948, t97065, t104606, t105021, t1203, t1214, t1294, t18090, t2149, t2150, t2151, t26906, t26937, t26969, t29109, t29122, t29158, t29179, t29217, t29264, t29282, t3552, t3575, t3601, t3738, t3783, t473, t5236, t5457, t7602, t7636, t7651, t7652, t8192, t8197, t8208, t8213, t96927, t97050, t97066, t97308, t97363, t97377, t97453);
        let t105107 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2245::<F>(t3596, t8190, t1214, t1248, t1287, t1769, t1774, t1775, t17992, t18103, t26884, t26906, t26922, t26949, t26976, t29122, t29158, t29175, t29178, t29204, t29220, t29278, t29304, t3576, t3588, t3601, t3769, t3790, t5457, t7632, t7636, t7637, t7643, t7651, t7652, t7659, t8213, t96910, t97419);
        let (t105121, t105122, t105155) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2246::<F>(t3601, t8208, t1794, t7627, t3153, t3555, t8190, t105021, t1214, t1215, t1294, t18030, t18070, t18073, t225, t26933, t26971, t26976, t29109, t29129, t29141, t29193, t29194, t29196, t29207, t29216, t29278, t3739, t3769, t3783, t460, t494, t5465, t7637, t7643, t7648, t7652, t8202, t96861, t96870, t96929, t96953, t97313, t97397);
        let (t105193, t105202, t105206) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2247::<F>(t1243, t29109, t105121, t73, t1032, t5412, t2148, t1214, t1248, t12657, t1287, t17968, t1829, t26889, t26895, t26922, t26949, t26963, t26999, t29118, t29159, t29167, t29271, t29275, t3584, t3790, t5237, t5458, t7632, t7635, t7636, t7637, t7643, t7652, t7654, t7659, t8197, t8198, t8201, t8208, t97422, t97425);
        let t105258 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2248::<F>(t1276, t2148, t3140, t5412, t1203, t1828, t1214, t104529, t105193, t2151, t26913, t26922, t26936, t26959, t26979, t27005, t27008, t27011, t27025, t29109, t29136, t29158, t29166, t29195, t29233, t29271, t29301, t3555, t3584, t3721, t5246, t5429, t5457, t5464, t7636, t7637, t7643, t7652, t7662, t8190, t8198, t8205, t96986, t97066, t97304);
    (t105057, t105107, t105122, t105155, t105202, t105206, t105258)
}

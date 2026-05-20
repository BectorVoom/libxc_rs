//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta928 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3021;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3022;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3023;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3024;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3025;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3026;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3027;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3028;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3029;
use chunk9::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3030;
use chunk10::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3031;
use chunk11::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3032;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta928<F: Float>(t24042: F, t359: F, t1024: F, t1082: F, t11788: F, t12127: F, t16506: F, t16523: F, t19483: F, t19556: F, t19566: F, t19572: F, t19580: F, t20119: F, t24075: F, t24084: F, t3204: F, t4757: F, t4954: F, t4996: F, t5009: F, t55991: F, t67599: F, t79084: F, t79175: F, t80028: F, t999: F, t1043: F, t1071: F, t1087: F, t1089: F, t12146: F, t12149: F, t12154: F, t19479: F, t19521: F, t19603: F, t23820: F, t23964: F, t24093: F, t24108: F, t24135: F, t3278: F, t3287: F, t43420: F, t4961: F, t4976: F, t4992: F, t55985: F, t6386: F, t67501: F, t78641: F, t79159: F, t1093: F, t12160: F, t15670: F, t1647: F, t16509: F, t19399: F, t19438: F, t19443: F, t19463: F, t19539: F, t19617: F, t20113: F, t23598: F, t23959: F, t24152: F, t3291: F, t3316: F, t342: F, t381: F, t4857: F, t4988: F, t4999: F, t5004: F, t5005: F, t6343: F, t79388: F, t11940: F, t15655: F, t16544: F, t1689: F, t19414: F, t19492: F, t19549: F, t20136: F, t24126: F, t24132: F, t24157: F, t3223: F, t43443: F, t4967: F, t4970: F, t55330: F, t55764: F, t6368: F, t65216: F, t65220: F, t67825: F, t78740: F, t12122: F, t19446: F, t19515: F, t19520: F, t19593: F, t20139: F, t23837: F, t3298: F, t43154: F, t43446: F, t4977: F, t4983: F, t4984: F, t55747: F, t55934: F, t6365: F, t67652: F, t67668: F, t73: F, t78554: F, t79116: F, t79505: F, t80264: F, t16449: F, t19477: F, t19557: F, t19594: F, t19597: F, t20133: F, t23992: F, t4893: F, t4981: F, t4982: F, t55701: F, t55988: F, t6244: F, t6371: F, t67969: F, t67972: F, t78873: F, t15780: F, t16381: F, t16502: F, t1692: F, t19457: F, t19498: F, t19509: F, t19612: F, t19856: F, t24089: F, t24104: F, t53877: F, t6383: F, t67927: F, t78831: F, t79480: F, t79500: F, t4930: F, t6305: F, t1651: F, t19453: F, t19526: F, t19573: F, t19576: F, t19608: F, t24123: F, t24147: F, t3299: F, t3304: F, t42261: F, t43384: F, t43598: F, t4772: F, t4964: F, t54695: F, t6258: F, t6362: F, t67595: F, t67644: F, t16552: F, t16553: F, t16560: F, t19450: F, t19503: F, t20146: F, t24141: F, t3318: F, t43453: F, t43524: F, t43528: F, t4866: F, t55569: F, t55570: F, t55593: F, t55594: F, t55732: F, t56049: F, t67714: F, t78496: F, t78812: F, t79180: F, t79275: F, t12167: F, t12168: F, t19380: F, t19569: F, t24138: F, t24144: F, t43432: F, t4980: F, t6235: F, t6379: F, t78826: F, t80341: F, t16410: F, t16520: F, t19484: F, t19602: F, t19607: F, t24090: F, t24098: F, t3317: F, t357: F, t43350: F, t43520: F, t4995: F, t4998: F, t55805: F, t55938: F, t55939: F, t19512: F, t20128: F, t24031: F, t24083: F, t43360: F, t55685: F, t6299: F, t65181: F, t66565: F, t79884: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t80425 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3021::<F>(t24042, t359, t1024, t1082, t11788, t12127, t16506, t16523, t19483, t19556, t19566, t19572, t19580, t20119, t24075, t24084, t3204, t4757, t4954, t4996, t5009, t55991, t67599, t79084, t79175, t80028, t999);
        let t80458 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3022::<F>(t1043, t1071, t1087, t1089, t12146, t12149, t12154, t19479, t19521, t19566, t19603, t23820, t23964, t24093, t24108, t24135, t3278, t3287, t43420, t4954, t4961, t4976, t4992, t55985, t6386, t67501, t78641, t79159);
        let t80490 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3023::<F>(t1024, t1093, t12160, t15670, t1647, t16509, t19399, t19438, t19443, t19463, t19539, t19566, t19617, t20113, t23598, t23959, t24152, t3204, t3291, t3316, t342, t381, t4857, t4988, t4999, t5004, t5005, t6343, t79388);
        let t80519 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3024::<F>(t1082, t11940, t12146, t12154, t15655, t16544, t1689, t19414, t19463, t19492, t19549, t20136, t24126, t24132, t24157, t3223, t43443, t4967, t4970, t55330, t55764, t6368, t65216, t65220, t67825, t78740);
        let t80557 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3025::<F>(t1082, t1089, t12122, t12149, t15670, t19446, t19515, t19520, t19593, t20139, t23837, t3287, t3298, t342, t43154, t43446, t4976, t4977, t4983, t4984, t55747, t55934, t6343, t6365, t67652, t67668, t73, t78554, t79116, t79505, t80264);
        let t80592 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3026::<F>(t1043, t1087, t1089, t12149, t15655, t16449, t19477, t19492, t19549, t19557, t19594, t19597, t20133, t23992, t24042, t3204, t4857, t4893, t4976, t4981, t4982, t4983, t55701, t55988, t55991, t6244, t6365, t6371, t67969, t67972, t73, t78873);
        let t80622 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3027::<F>(t1082, t1089, t15670, t15780, t16381, t16502, t16544, t1692, t19457, t19498, t19509, t19612, t19856, t24089, t24104, t3204, t3278, t3287, t4977, t4981, t53877, t6383, t67927, t78831, t79480, t79500);
        let (t80640, t80654) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3028::<F>(t4930, t6305, t1024, t16449, t1651, t19453, t19521, t19526, t19556, t19573, t19576, t19603, t19608, t24123, t24126, t24147, t3299, t3304, t42261, t43384, t43598, t4772, t4964, t54695, t6258, t6362, t67595, t67644, t67652);
        let t80691 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3029::<F>(t1043, t1089, t12127, t16544, t16552, t16553, t16560, t19450, t19503, t19580, t20146, t24141, t3287, t3318, t43453, t43524, t43528, t4866, t4977, t55569, t55570, t55593, t55594, t55732, t56049, t67714, t78496, t78812, t79180, t79275, t999);
        let t80724 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3030::<F>(t1024, t1082, t11788, t12122, t12167, t12168, t16381, t16502, t19380, t19526, t19569, t19573, t19576, t19612, t20136, t23964, t24138, t24144, t3204, t3291, t3304, t43432, t4980, t4984, t5004, t6235, t6379, t78826, t79275, t80341);
        let t80764 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3031::<F>(t1043, t16410, t1647, t16520, t16544, t16553, t19484, t19498, t19569, t19602, t19607, t24090, t24098, t3223, t3317, t3318, t357, t43350, t43520, t4984, t4995, t4996, t4998, t4999, t55805, t55938, t55939, t6235, t78496, t78812, t78873, t80640, t999);
        let t80798 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3032::<F>(t1087, t1089, t11940, t15670, t19453, t19484, t19512, t19608, t20128, t24031, t24083, t24138, t3291, t43360, t43446, t4857, t4866, t4930, t4964, t4996, t55685, t6299, t6343, t6365, t65181, t66565, t67714, t67927, t79884);
    (t80425, t80458, t80490, t80519, t80557, t80592, t80622, t80654, t80691, t80724, t80764, t80798)
}

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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta928(t24042: f64, t359: f64, t1024: f64, t1082: f64, t11788: f64, t12127: f64, t16506: f64, t16523: f64, t19483: f64, t19556: f64, t19566: f64, t19572: f64, t19580: f64, t20119: f64, t24075: f64, t24084: f64, t3204: f64, t4757: f64, t4954: f64, t4996: f64, t5009: f64, t55991: f64, t67599: f64, t79084: f64, t79175: f64, t80028: f64, t999: f64, t1043: f64, t1071: f64, t1087: f64, t1089: f64, t12146: f64, t12149: f64, t12154: f64, t19479: f64, t19521: f64, t19603: f64, t23820: f64, t23964: f64, t24093: f64, t24108: f64, t24135: f64, t3278: f64, t3287: f64, t43420: f64, t4961: f64, t4976: f64, t4992: f64, t55985: f64, t6386: f64, t67501: f64, t78641: f64, t79159: f64, t1093: f64, t12160: f64, t15670: f64, t1647: f64, t16509: f64, t19399: f64, t19438: f64, t19443: f64, t19463: f64, t19539: f64, t19617: f64, t20113: f64, t23598: f64, t23959: f64, t24152: f64, t3291: f64, t3316: f64, t342: f64, t381: f64, t4857: f64, t4988: f64, t4999: f64, t5004: f64, t5005: f64, t6343: f64, t79388: f64, t11940: f64, t15655: f64, t16544: f64, t1689: f64, t19414: f64, t19492: f64, t19549: f64, t20136: f64, t24126: f64, t24132: f64, t24157: f64, t3223: f64, t43443: f64, t4967: f64, t4970: f64, t55330: f64, t55764: f64, t6368: f64, t65216: f64, t65220: f64, t67825: f64, t78740: f64, t12122: f64, t19446: f64, t19515: f64, t19520: f64, t19593: f64, t20139: f64, t23837: f64, t3298: f64, t43154: f64, t43446: f64, t4977: f64, t4983: f64, t4984: f64, t55747: f64, t55934: f64, t6365: f64, t67652: f64, t67668: f64, t73: f64, t78554: f64, t79116: f64, t79505: f64, t80264: f64, t16449: f64, t19477: f64, t19557: f64, t19594: f64, t19597: f64, t20133: f64, t23992: f64, t4893: f64, t4981: f64, t4982: f64, t55701: f64, t55988: f64, t6244: f64, t6371: f64, t67969: f64, t67972: f64, t78873: f64, t15780: f64, t16381: f64, t16502: f64, t1692: f64, t19457: f64, t19498: f64, t19509: f64, t19612: f64, t19856: f64, t24089: f64, t24104: f64, t53877: f64, t6383: f64, t67927: f64, t78831: f64, t79480: f64, t79500: f64, t4930: f64, t6305: f64, t1651: f64, t19453: f64, t19526: f64, t19573: f64, t19576: f64, t19608: f64, t24123: f64, t24147: f64, t3299: f64, t3304: f64, t42261: f64, t43384: f64, t43598: f64, t4772: f64, t4964: f64, t54695: f64, t6258: f64, t6362: f64, t67595: f64, t67644: f64, t16552: f64, t16553: f64, t16560: f64, t19450: f64, t19503: f64, t20146: f64, t24141: f64, t3318: f64, t43453: f64, t43524: f64, t43528: f64, t4866: f64, t55569: f64, t55570: f64, t55593: f64, t55594: f64, t55732: f64, t56049: f64, t67714: f64, t78496: f64, t78812: f64, t79180: f64, t79275: f64, t12167: f64, t12168: f64, t19380: f64, t19569: f64, t24138: f64, t24144: f64, t43432: f64, t4980: f64, t6235: f64, t6379: f64, t78826: f64, t80341: f64, t16410: f64, t16520: f64, t19484: f64, t19602: f64, t19607: f64, t24090: f64, t24098: f64, t3317: f64, t357: f64, t43350: f64, t43520: f64, t4995: f64, t4998: f64, t55805: f64, t55938: f64, t55939: f64, t19512: f64, t20128: f64, t24031: f64, t24083: f64, t43360: f64, t55685: f64, t6299: f64, t65181: f64, t66565: f64, t79884: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t80425 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3021(t24042, t359, t1024, t1082, t11788, t12127, t16506, t16523, t19483, t19556, t19566, t19572, t19580, t20119, t24075, t24084, t3204, t4757, t4954, t4996, t5009, t55991, t67599, t79084, t79175, t80028, t999);
        let t80458 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3022(t1043, t1071, t1087, t1089, t12146, t12149, t12154, t19479, t19521, t19566, t19603, t23820, t23964, t24093, t24108, t24135, t3278, t3287, t43420, t4954, t4961, t4976, t4992, t55985, t6386, t67501, t78641, t79159);
        let t80490 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3023(t1024, t1093, t12160, t15670, t1647, t16509, t19399, t19438, t19443, t19463, t19539, t19566, t19617, t20113, t23598, t23959, t24152, t3204, t3291, t3316, t342, t381, t4857, t4988, t4999, t5004, t5005, t6343, t79388);
        let t80519 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3024(t1082, t11940, t12146, t12154, t15655, t16544, t1689, t19414, t19463, t19492, t19549, t20136, t24126, t24132, t24157, t3223, t43443, t4967, t4970, t55330, t55764, t6368, t65216, t65220, t67825, t78740);
        let t80557 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3025(t1082, t1089, t12122, t12149, t15670, t19446, t19515, t19520, t19593, t20139, t23837, t3287, t3298, t342, t43154, t43446, t4976, t4977, t4983, t4984, t55747, t55934, t6343, t6365, t67652, t67668, t73, t78554, t79116, t79505, t80264);
        let t80592 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3026(t1043, t1087, t1089, t12149, t15655, t16449, t19477, t19492, t19549, t19557, t19594, t19597, t20133, t23992, t24042, t3204, t4857, t4893, t4976, t4981, t4982, t4983, t55701, t55988, t55991, t6244, t6365, t6371, t67969, t67972, t73, t78873);
        let t80622 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3027(t1082, t1089, t15670, t15780, t16381, t16502, t16544, t1692, t19457, t19498, t19509, t19612, t19856, t24089, t24104, t3204, t3278, t3287, t4977, t4981, t53877, t6383, t67927, t78831, t79480, t79500);
        let (t80640, t80654) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3028(t4930, t6305, t1024, t16449, t1651, t19453, t19521, t19526, t19556, t19573, t19576, t19603, t19608, t24123, t24126, t24147, t3299, t3304, t42261, t43384, t43598, t4772, t4964, t54695, t6258, t6362, t67595, t67644, t67652);
        let t80691 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3029(t1043, t1089, t12127, t16544, t16552, t16553, t16560, t19450, t19503, t19580, t20146, t24141, t3287, t3318, t43453, t43524, t43528, t4866, t4977, t55569, t55570, t55593, t55594, t55732, t56049, t67714, t78496, t78812, t79180, t79275, t999);
        let t80724 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3030(t1024, t1082, t11788, t12122, t12167, t12168, t16381, t16502, t19380, t19526, t19569, t19573, t19576, t19612, t20136, t23964, t24138, t24144, t3204, t3291, t3304, t43432, t4980, t4984, t5004, t6235, t6379, t78826, t79275, t80341);
        let t80764 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3031(t1043, t16410, t1647, t16520, t16544, t16553, t19484, t19498, t19569, t19602, t19607, t24090, t24098, t3223, t3317, t3318, t357, t43350, t43520, t4984, t4995, t4996, t4998, t4999, t55805, t55938, t55939, t6235, t78496, t78812, t78873, t80640, t999);
        let t80798 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3032(t1087, t1089, t11940, t15670, t19453, t19484, t19512, t19608, t20128, t24031, t24083, t24138, t3291, t43360, t43446, t4857, t4866, t4930, t4964, t4996, t55685, t6299, t6343, t6365, t65181, t66565, t67714, t67927, t79884);
    (t80425, t80458, t80490, t80519, t80557, t80592, t80622, t80654, t80691, t80724, t80764, t80798)
}

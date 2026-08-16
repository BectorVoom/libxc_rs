//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta926 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3003;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3004;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3005;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3006;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3007;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3008;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta926<F: Float>(t1043: F, t1668: F, t357: F, t11660: F, t11703: F, t11774: F, t11933: F, t15618: F, t15758: F, t15917: F, t16067: F, t16095: F, t18936: F, t19501: F, t19572: F, t19645: F, t19776: F, t19778: F, t19782: F, t19971: F, t19992: F, t19997: F, t23470: F, t23474: F, t23900: F, t23904: F, t23994: F, t24007: F, t3091: F, t3092: F, t3117: F, t4186: F, t42410: F, t43050: F, t43082: F, t4786: F, t4873: F, t4892: F, t4899: F, t53511: F, t54578: F, t54599: F, t6268: F, t65144: F, t66689: F, t66814: F, t11941: F, t127: F, t24032: F, t371: F, t15671: F, t20016: F, t1025: F, t24022: F, t1011: F, t16170: F, t1665: F, t19981: F, t19985: F, t24034: F, t372: F, t42328: F, t42996: F, t43161: F, t4915: F, t54404: F, t54648: F, t54687: F, t54733: F, t6266: F, t6339: F, t66395: F, t66822: F, t66860: F, t66943: F, t67215: F, t77541: F, t77545: F, t1592: F, t4866: F, t15993: F, t23499: F, t11875: F, t11922: F, t24012: F, t11632: F, t15689: F, t15691: F, t15696: F, t16226: F, t19996: F, t20038: F, t20039: F, t20078: F, t23984: F, t24013: F, t3155: F, t3162: F, t3241: F, t42675: F, t4574: F, t53741: F, t54811: F, t55331: F, t66187: F, t66702: F, t66777: F, t66951: F, t66966: F, t66972: F, t66981: F, t67006: F, t67015: F, t78524: F, t79247: F, t23958: F, t993: F, t225: F, t366: F, t1028: F, t15584: F, t15701: F, t16222: F, t18941: F, t19725: F, t19738: F, t19773: F, t19973: F, t23857: F, t23939: F, t4181: F, t42155: F, t4854: F, t54943: F, t55011: F, t63287: F, t63363: F, t67025: F, t67044: F, t67048: F, t67072: F, t67516: F, t78901: F, t79097: F, t79610: F, t20020: F, t4858: F, t140: F, t23877: F, t24031: F, t15823: F, t20029: F, t1045: F, t15700: F, t19625: F, t23878: F, t24024: F, t3181: F, t3211: F, t43069: F, t43291: F, t4782: F, t55034: F, t6299: F, t66306: F, t67152: F, t67186: F, t67195: F, t67199: F, t67206: F, t67213: F, t67237: F, t67249: F, t67253: F, t11710: F, t23899: F, t15987: F, t23503: F, t4845: F, t15656: F, t16089: F, t18946: F, t19770: F, t23839: F, t23911: F, t23945: F, t42690: F, t43238: F, t43285: F, t55062: F, t55065: F, t55155: F, t6100: F, t6271: F, t6278: F, t66434: F, t67264: F, t67301: F, t79410: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t79703, t79723) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3003::<F>(t1043, t1668, t357, t11660, t11703, t11774, t11933, t15618, t15758, t15917, t16067, t16095, t18936, t19501, t19572, t19645, t19776, t19778, t19782, t19971, t19992, t19997, t23470, t23474, t23900, t23904, t23994, t24007, t3091, t3092, t3117, t4186, t42410, t43050, t43082, t4786, t4873, t4892, t4899, t53511, t54578, t54599, t6268, t65144, t66689, t66814);
        let t79768 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3004::<F>(t11941, t127, t24032, t371, t15671, t20016, t1025, t24022, t1011, t11774, t16170, t1665, t1668, t19981, t19985, t24034, t3091, t3092, t372, t42328, t42996, t43161, t4915, t54404, t54648, t54687, t54733, t6266, t6339, t66395, t66689, t66822, t66860, t66943, t67215, t77541, t77545);
        let (t79770, t79822) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3005::<F>(t1592, t4866, t1011, t15993, t23499, t11875, t11922, t24012, t11632, t11703, t11774, t15689, t15691, t15696, t16095, t16226, t18936, t19985, t19996, t20038, t20039, t20078, t23984, t24013, t3092, t3155, t3162, t3241, t42675, t43082, t4574, t4873, t53741, t54811, t55331, t6266, t66187, t66702, t66777, t66951, t66966, t66972, t66981, t67006, t67015, t78524, t79247);
        let (t79862, t79863, t79870) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3006::<F>(t23958, t993, t225, t366, t1028, t11703, t11774, t15584, t15689, t15696, t15701, t16095, t16222, t1665, t18941, t19725, t19738, t19773, t19973, t23857, t23939, t3092, t4181, t42155, t42410, t43082, t4786, t4854, t54943, t55011, t63287, t63363, t67025, t67044, t67048, t67072, t67516, t78524, t78901, t79097, t79610);
        let (t79884, t79907) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3007::<F>(t20020, t4858, t1011, t140, t23877, t1043, t24031, t15823, t20029, t1045, t15696, t15700, t19625, t19981, t23878, t24024, t3117, t3181, t3211, t3241, t372, t42328, t43069, t43291, t4782, t55034, t6299, t66306, t67152, t67186, t67195, t67199, t67206, t67213, t67237, t67249, t67253);
        let t79951 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3008::<F>(t11710, t23899, t4892, t1011, t15987, t23503, t19773, t4845, t11875, t15656, t16089, t18946, t19770, t23839, t23911, t23945, t24007, t3091, t3092, t3117, t3162, t3241, t357, t42690, t43238, t43285, t4858, t4866, t4873, t55062, t55065, t55155, t6100, t6271, t6278, t66434, t66702, t67264, t67301, t79410);
    (t79703, t79723, t79768, t79770, t79822, t79862, t79863, t79870, t79884, t79907, t79951)
}

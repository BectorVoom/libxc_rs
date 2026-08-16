//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta565 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1714;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1715;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1716;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1717;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1718;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1719;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1720;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1721;
use chunk8::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1722;
use chunk9::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1723;
use chunk10::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1724;
use chunk11::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1725;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta565<F: Float>(t1024: F, t1082: F, t1087: F, t1089: F, t11940: F, t12167: F, t12168: F, t19463: F, t19556: F, t19566: F, t24167: F, t3299: F, t3304: F, t378: F, t381: F, t4857: F, t6235: F, t6258: F, t6371: F, t6375: F, t6383: F, t6389: F, t67725: F, t88646: F, t88675: F, t88998: F, t89490: F, t89503: F, t12078: F, t12079: F, t16502: F, t16584: F, t1678: F, t23820: F, t24098: F, t24132: F, t24138: F, t24141: F, t24152: F, t3287: F, t342: F, t380: F, t55988: F, t55991: F, t6368: F, t6379: F, t89245: F, t89355: F, t12122: F, t12127: F, t16552: F, t16553: F, t16559: F, t16560: F, t1668: F, t19450: F, t19603: F, t24083: F, t24090: F, t24135: F, t3318: F, t43520: F, t43524: F, t4893: F, t4981: F, t4982: F, t55732: F, t6299: F, t80264: F, t88794: F, t88804: F, t12149: F, t1685: F, t1692: F, t19569: F, t19608: F, t23959: F, t24084: F, t24123: F, t24126: F, t55599: F, t55747: F, t55887: F, t79863: F, t88885: F, t89035: F, t89158: F, t89240: F, t88694: F, t15670: F, t16509: F, t16544: F, t23598: F, t24079: F, t24093: F, t24112: F, t24144: F, t24147: F, t43347: F, t43352: F, t43537: F, t43538: F, t5004: F, t53877: F, t55899: F, t56017: F, t6343: F, t6386: F, t67790: F, t12052: F, t1647: F, t1651: F, t16566: F, t23964: F, t24031: F, t24075: F, t24162: F, t3204: F, t43341: F, t43401: F, t43402: F, t43438: F, t43456: F, t43472: F, t43473: F, t80350: F, t80396: F, t89084: F, t19526: F, t24104: F, t24116: F, t24157: F, t43446: F, t4954: F, t4996: F, t56049: F, t6244: F, t6362: F, t6365: F, t67501: F, t67652: F, t67714: F, t67927: F, t78873: F, t88948: F, t6350: F, t1076: F, t1079: F, t11201: F, t16284: F, t16600: F, t16603: F, t16604: F, t1695: F, t20175: F, t20191: F, t20204: F, t23583: F, t23599: F, t23616: F, t24048: F, t24061: F, t24068: F, t24178: F, t3058: F, t42067: F, t4747: F, t4752: F, t53015: F, t53160: F, t6259: F, t6392: F, t6393: F, t88815: F, t89507: F, t995: F, t996: F, t6396: F, t6400: F, t1102: F, t198: F, t3336: F, t336: F, t41937: F, t88510: F, t88562: F, t88564: F, t88567: F, t88607: F, t88682: F, t88986: F, t88989: F, t88991: F, t88993: F, t88995: F, t89397: F, t89437: F, t30: F, t265: F, t393: F, t87990: F, t88042: F, t88577: F, t88603: F, t1468: F, t1469: F, t1587: F, t1704: F, t22670: F, t22671: F, t23436: F, t24192: F, t395: F, t45: F, t5824: F, t5825: F, t6084: F, t6405: F, t87125: F, t87126: F, dens_threshold: F, rho0: F, zeta_threshold: F, t6587: F, t20292: F, t12305: F, t128: F, t20297: F, t3360: F) -> (F, F, F, F, F, F, F) {
        let t89536 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1714::<F>(t1024, t1082, t1087, t1089, t11940, t12167, t12168, t19463, t19556, t19566, t24167, t3299, t3304, t378, t381, t4857, t6235, t6258, t6371, t6375, t6383, t6389, t67725, t88646, t88675, t88998, t89490, t89503);
        let t89565 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1715::<F>(t1087, t1089, t12078, t12079, t16502, t16584, t1678, t19463, t19566, t23820, t24098, t24132, t24138, t24141, t24152, t3287, t342, t380, t4857, t55988, t55991, t6368, t6379, t89245, t89355, t89503);
        let t89603 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1716::<F>(t12079, t12122, t12127, t12168, t16502, t16552, t16553, t16559, t16560, t1668, t19450, t19603, t23820, t24083, t24090, t24135, t24141, t3304, t3318, t43520, t43524, t4893, t4981, t4982, t55732, t6299, t80264, t88794, t88804);
        let t89632 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1717::<F>(t1024, t1082, t1089, t12149, t1685, t1692, t19569, t19608, t23959, t24084, t24123, t24126, t3287, t55599, t55747, t55887, t79863, t88885, t89035, t89158, t89240);
        let (t89647, t89663) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1718::<F>(t378, t88694, t1024, t1087, t1089, t15670, t16509, t16544, t23598, t24079, t24093, t24112, t24132, t24135, t24144, t24147, t43347, t43352, t43537, t43538, t5004, t53877, t55899, t56017, t6299, t6343, t6386, t67790);
        let t89697 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1719::<F>(t1024, t11940, t12052, t15670, t1647, t1651, t16566, t19450, t23964, t24031, t24075, t24162, t3204, t3304, t3318, t43341, t43401, t43402, t43438, t43456, t43472, t43473, t5004, t6299, t80350, t80396, t88794, t89084, t89647);
        let t89725 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1720::<F>(t1089, t19526, t19556, t24083, t24090, t24104, t24116, t24138, t24157, t3204, t43446, t4857, t4954, t4996, t56049, t6244, t6362, t6365, t67501, t67652, t67714, t67927, t78873, t88948);
        let t89740 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1721::<F>(t6350, t1076, t1079, t11201, t16284, t16600, t16603, t16604, t1695, t20175, t20191, t20204, t23583, t23599, t23616, t24031, t24048, t24061, t24068, t24178, t3058, t42067, t4747, t4752, t53015, t53160, t6258, t6259, t6392, t6393, t88815, t89158, t89507, t89536, t89565, t89603, t89632, t89663, t89697, t89725, t995, t996);
        let t89756 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1722::<F>(t6396, t6400, t1102, t198, t3336, t336, t41937, t88510, t88562, t88564, t88567, t88607, t88682, t88986, t88989, t88991, t88993, t88995, t89397, t89437, t89740);
        let t89771 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1723::<F>(t30, t265, t393, t87990, t88042, t88577, t88603, t89756, t1468, t1469, t1587, t1704, t22670, t22671, t23436, t24192, t395, t45, t5824, t5825, t6084, t6405, t87125, t87126, dens_threshold, rho0, zeta_threshold);
        let (t89780, t89808, t89822, t89824) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1724::<F>(t87125, t6587, t20292, t5825, t12305, t128);
        let (t89826, t89828) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1725::<F>(t20297, t5825, t128, t3360);
    (t89771, t89780, t89808, t89822, t89824, t89826, t89828)
}

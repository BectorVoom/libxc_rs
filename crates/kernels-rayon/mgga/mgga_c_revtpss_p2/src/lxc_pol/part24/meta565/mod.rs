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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta565(t1024: f64, t1082: f64, t1087: f64, t1089: f64, t11940: f64, t12167: f64, t12168: f64, t19463: f64, t19556: f64, t19566: f64, t24167: f64, t3299: f64, t3304: f64, t378: f64, t381: f64, t4857: f64, t6235: f64, t6258: f64, t6371: f64, t6375: f64, t6383: f64, t6389: f64, t67725: f64, t88646: f64, t88675: f64, t88998: f64, t89490: f64, t89503: f64, t12078: f64, t12079: f64, t16502: f64, t16584: f64, t1678: f64, t23820: f64, t24098: f64, t24132: f64, t24138: f64, t24141: f64, t24152: f64, t3287: f64, t342: f64, t380: f64, t55988: f64, t55991: f64, t6368: f64, t6379: f64, t89245: f64, t89355: f64, t12122: f64, t12127: f64, t16552: f64, t16553: f64, t16559: f64, t16560: f64, t1668: f64, t19450: f64, t19603: f64, t24083: f64, t24090: f64, t24135: f64, t3318: f64, t43520: f64, t43524: f64, t4893: f64, t4981: f64, t4982: f64, t55732: f64, t6299: f64, t80264: f64, t88794: f64, t88804: f64, t12149: f64, t1685: f64, t1692: f64, t19569: f64, t19608: f64, t23959: f64, t24084: f64, t24123: f64, t24126: f64, t55599: f64, t55747: f64, t55887: f64, t79863: f64, t88885: f64, t89035: f64, t89158: f64, t89240: f64, t88694: f64, t15670: f64, t16509: f64, t16544: f64, t23598: f64, t24079: f64, t24093: f64, t24112: f64, t24144: f64, t24147: f64, t43347: f64, t43352: f64, t43537: f64, t43538: f64, t5004: f64, t53877: f64, t55899: f64, t56017: f64, t6343: f64, t6386: f64, t67790: f64, t12052: f64, t1647: f64, t1651: f64, t16566: f64, t23964: f64, t24031: f64, t24075: f64, t24162: f64, t3204: f64, t43341: f64, t43401: f64, t43402: f64, t43438: f64, t43456: f64, t43472: f64, t43473: f64, t80350: f64, t80396: f64, t89084: f64, t19526: f64, t24104: f64, t24116: f64, t24157: f64, t43446: f64, t4954: f64, t4996: f64, t56049: f64, t6244: f64, t6362: f64, t6365: f64, t67501: f64, t67652: f64, t67714: f64, t67927: f64, t78873: f64, t88948: f64, t6350: f64, t1076: f64, t1079: f64, t11201: f64, t16284: f64, t16600: f64, t16603: f64, t16604: f64, t1695: f64, t20175: f64, t20191: f64, t20204: f64, t23583: f64, t23599: f64, t23616: f64, t24048: f64, t24061: f64, t24068: f64, t24178: f64, t3058: f64, t42067: f64, t4747: f64, t4752: f64, t53015: f64, t53160: f64, t6259: f64, t6392: f64, t6393: f64, t88815: f64, t89507: f64, t995: f64, t996: f64, t6396: f64, t6400: f64, t1102: f64, t198: f64, t3336: f64, t336: f64, t41937: f64, t88510: f64, t88562: f64, t88564: f64, t88567: f64, t88607: f64, t88682: f64, t88986: f64, t88989: f64, t88991: f64, t88993: f64, t88995: f64, t89397: f64, t89437: f64, t30: f64, t265: f64, t393: f64, t87990: f64, t88042: f64, t88577: f64, t88603: f64, t1468: f64, t1469: f64, t1587: f64, t1704: f64, t22670: f64, t22671: f64, t23436: f64, t24192: f64, t395: f64, t45: f64, t5824: f64, t5825: f64, t6084: f64, t6405: f64, t87125: f64, t87126: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t6587: f64, t20292: f64, t12305: f64, t128: f64, t20297: f64, t3360: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t89536 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1714(t1024, t1082, t1087, t1089, t11940, t12167, t12168, t19463, t19556, t19566, t24167, t3299, t3304, t378, t381, t4857, t6235, t6258, t6371, t6375, t6383, t6389, t67725, t88646, t88675, t88998, t89490, t89503);
        let t89565 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1715(t1087, t1089, t12078, t12079, t16502, t16584, t1678, t19463, t19566, t23820, t24098, t24132, t24138, t24141, t24152, t3287, t342, t380, t4857, t55988, t55991, t6368, t6379, t89245, t89355, t89503);
        let t89603 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1716(t12079, t12122, t12127, t12168, t16502, t16552, t16553, t16559, t16560, t1668, t19450, t19603, t23820, t24083, t24090, t24135, t24141, t3304, t3318, t43520, t43524, t4893, t4981, t4982, t55732, t6299, t80264, t88794, t88804);
        let t89632 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1717(t1024, t1082, t1089, t12149, t1685, t1692, t19569, t19608, t23959, t24084, t24123, t24126, t3287, t55599, t55747, t55887, t79863, t88885, t89035, t89158, t89240);
        let (t89647, t89663) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1718(t378, t88694, t1024, t1087, t1089, t15670, t16509, t16544, t23598, t24079, t24093, t24112, t24132, t24135, t24144, t24147, t43347, t43352, t43537, t43538, t5004, t53877, t55899, t56017, t6299, t6343, t6386, t67790);
        let t89697 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1719(t1024, t11940, t12052, t15670, t1647, t1651, t16566, t19450, t23964, t24031, t24075, t24162, t3204, t3304, t3318, t43341, t43401, t43402, t43438, t43456, t43472, t43473, t5004, t6299, t80350, t80396, t88794, t89084, t89647);
        let t89725 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1720(t1089, t19526, t19556, t24083, t24090, t24104, t24116, t24138, t24157, t3204, t43446, t4857, t4954, t4996, t56049, t6244, t6362, t6365, t67501, t67652, t67714, t67927, t78873, t88948);
        let t89740 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1721(t6350, t1076, t1079, t11201, t16284, t16600, t16603, t16604, t1695, t20175, t20191, t20204, t23583, t23599, t23616, t24031, t24048, t24061, t24068, t24178, t3058, t42067, t4747, t4752, t53015, t53160, t6258, t6259, t6392, t6393, t88815, t89158, t89507, t89536, t89565, t89603, t89632, t89663, t89697, t89725, t995, t996);
        let t89756 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1722(t6396, t6400, t1102, t198, t3336, t336, t41937, t88510, t88562, t88564, t88567, t88607, t88682, t88986, t88989, t88991, t88993, t88995, t89397, t89437, t89740);
        let t89771 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1723(t30, t265, t393, t87990, t88042, t88577, t88603, t89756, t1468, t1469, t1587, t1704, t22670, t22671, t23436, t24192, t395, t45, t5824, t5825, t6084, t6405, t87125, t87126, dens_threshold, rho0, zeta_threshold);
        let (t89780, t89808, t89822, t89824) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1724(t87125, t6587, t20292, t5825, t12305, t128);
        let (t89826, t89828) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1725(t20297, t5825, t128, t3360);
    (t89771, t89780, t89808, t89822, t89824, t89826, t89828)
}

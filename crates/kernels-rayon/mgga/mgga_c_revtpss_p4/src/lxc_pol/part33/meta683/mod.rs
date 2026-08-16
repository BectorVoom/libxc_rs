//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta683 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2240;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2241;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2242;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2243;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2244;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2245;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2246;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2247;
use chunk8::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2248;
use chunk9::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2249;
use chunk10::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2250;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta683(t104624: f64, t104626: f64, t104640: f64, t104651: f64, t104653: f64, t20806: f64, t20811: f64, t20876: f64, t21153: f64, t21166: f64, t21259: f64, t26870: f64, t26880: f64, t29100: f64, t6690: f64, t7624: f64, t97182: f64, t6601: f64, t7623: f64, t21188: f64, t26844: f64, t104658: f64, t104680: f64, t104732: f64, t1266: f64, t17307: f64, t1808: f64, t20864: f64, t20978: f64, t21111: f64, t26873: f64, t29020: f64, t29037: f64, t5287: f64, t5304: f64, t5386: f64, t6625: f64, t97149: f64, t104706: f64, t4890: f64, t104689: f64, t104691: f64, t104752: f64, t104856: f64, t104943: f64, t20771: f64, t20907: f64, t20914: f64, t20938: f64, t20941: f64, t20947: f64, t29083: f64, t3782: f64, t5270: f64, t5299: f64, t5335: f64, t6635: f64, t97129: f64, t97174: f64, t21233: f64, t5378: f64, t21090: f64, t26867: f64, t104703: f64, t104708: f64, t104715: f64, t104774: f64, t17183: f64, t20959: f64, t20963: f64, t21030: f64, t21246: f64, t29040: f64, t29096: f64, t5348: f64, t5354: f64, t5397: f64, t97141: f64, t97261: f64, t29019: f64, t5273: f64, t20973: f64, t1785: f64, t29082: f64, t104727: f64, t104739: f64, t104742: f64, t1252: f64, t21200: f64, t21267: f64, t26852: f64, t3670: f64, t6631: f64, t6673: f64, t6683: f64, t97206: f64, t21192: f64, t104636: f64, t104677: f64, t104756: f64, t104768: f64, t104834: f64, t1797: f64, t20825: f64, t20903: f64, t20982: f64, t20986: f64, t29010: f64, t5279: f64, t7618: f64, t1219: f64, t30800: f64, t1241: f64, t21100: f64, t7616: f64, t1256: f64, t30789: f64, t104770: f64, t1230: f64, t20802: f64, t21095: f64, t21300: f64, t21334: f64, t2138: f64, t29097: f64, t30815: f64, t484: f64, t5261: f64, t6619: f64, t8184: f64, t97177: f64, t97250: f64, t20786: f64, t26849: f64, t5265: f64, t20819: f64, t7617: f64, t104696: f64, t104793: f64, t104815: f64, t104817: f64, t104825: f64, t104828: f64, t104833: f64, t20797: f64, t21046: f64, t30799: f64, t800: f64, t21270: f64, t2137: f64, t467: f64, t20926: f64, t104647: f64, t104844: f64, t104924: f64, t1227: f64, t1791: f64, t17934: f64, t20838: f64, t20923: f64, t20934: f64, t29062: f64, t5320: f64, t5343: f64, t6611: f64, t97292: f64, t20850: f64, t29086: f64, t5362: f64, t104863: f64, t104872: f64, t104916: f64, t104946: f64, t1238: f64, t20858: f64, t20952: f64, t21042: f64, t21310: f64, t3767: f64, t97179: f64, t21169: f64, t7607: f64, t816: f64, t8171: f64, t104894: f64, t104905: f64, t20266: f64, t20293: f64, t20306: f64, t20310: f64, t20868: f64, t21004: f64, t21184: f64, t29047: f64, t29048: f64, t29054: f64, t29055: f64, t57549: f64, t6679: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t112175 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2240(t104624, t104626, t104640, t104651, t104653, t20806, t20811, t20876, t21153, t21166, t21259, t26870, t26880, t29100, t6690, t7624, t97182);
        let t112200 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2241(t6601, t7623, t21188, t26844, t104658, t104680, t104732, t1266, t17307, t1808, t20864, t20978, t21111, t26873, t29020, t29037, t5287, t5304, t5386, t6625, t7624, t97149);
        let (t112220, t112224) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2242(t104706, t4890, t104689, t104691, t104752, t104856, t104943, t20771, t20907, t20914, t20938, t20941, t20947, t26880, t29083, t3782, t5270, t5299, t5335, t6635, t7624, t97129, t97174);
        let t112249 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2243(t21233, t7624, t29083, t5378, t21090, t26867, t104703, t104708, t104715, t104774, t17183, t20959, t20963, t21030, t21246, t29040, t29096, t5335, t5348, t5354, t5397, t97141, t97261);
        let t112278 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2244(t29019, t5273, t20973, t7624, t1785, t29082, t104727, t104739, t104742, t1252, t1266, t1808, t21200, t21267, t26852, t29037, t29040, t3670, t5386, t5397, t6631, t6673, t6683, t97206);
        let t112299 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2245(t21192, t7624, t104636, t104677, t104756, t104768, t104834, t1797, t20825, t20903, t20982, t20986, t26880, t29010, t5279, t5287, t5299, t7618);
        let t112327 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2246(t1219, t30800, t1241, t21100, t7616, t1256, t30789, t104770, t1230, t1252, t20802, t21095, t21300, t21334, t2138, t26870, t29040, t29097, t30815, t484, t5261, t6619, t8184, t97177, t97250);
        let t112342 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2247(t29037, t5378, t20786, t26849, t29010, t5265, t20819, t7617, t104696, t104793, t104815, t104817, t104825, t104828, t104833, t1252, t20797, t21046, t97261);
        let t112372 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2248(t30799, t800, t21270, t2137, t467, t20926, t26870, t104647, t104752, t104844, t104924, t1227, t1266, t1791, t17934, t20838, t20923, t20934, t29062, t29096, t29100, t5279, t5320, t5343, t6611, t97174, t97292);
        let t112395 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2249(t20850, t2138, t29086, t5362, t104703, t104863, t104872, t104916, t104946, t112220, t1238, t1791, t20858, t20952, t21042, t21310, t26870, t29097, t3767, t5320, t5343, t5354, t97179);
        let (t112404, t112424) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2250(t21169, t7607, t816, t8171, t104894, t104905, t20266, t20293, t20306, t20310, t20868, t21004, t21184, t26852, t26880, t29047, t29048, t29054, t29055, t57549, t6679, t7624, t97179);
    (t112175, t112200, t112224, t112249, t112278, t112299, t112327, t112342, t112372, t112395, t112404, t112424)
}

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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta683<F: Float>(t104624: F, t104626: F, t104640: F, t104651: F, t104653: F, t20806: F, t20811: F, t20876: F, t21153: F, t21166: F, t21259: F, t26870: F, t26880: F, t29100: F, t6690: F, t7624: F, t97182: F, t6601: F, t7623: F, t21188: F, t26844: F, t104658: F, t104680: F, t104732: F, t1266: F, t17307: F, t1808: F, t20864: F, t20978: F, t21111: F, t26873: F, t29020: F, t29037: F, t5287: F, t5304: F, t5386: F, t6625: F, t97149: F, t104706: F, t4890: F, t104689: F, t104691: F, t104752: F, t104856: F, t104943: F, t20771: F, t20907: F, t20914: F, t20938: F, t20941: F, t20947: F, t29083: F, t3782: F, t5270: F, t5299: F, t5335: F, t6635: F, t97129: F, t97174: F, t21233: F, t5378: F, t21090: F, t26867: F, t104703: F, t104708: F, t104715: F, t104774: F, t17183: F, t20959: F, t20963: F, t21030: F, t21246: F, t29040: F, t29096: F, t5348: F, t5354: F, t5397: F, t97141: F, t97261: F, t29019: F, t5273: F, t20973: F, t1785: F, t29082: F, t104727: F, t104739: F, t104742: F, t1252: F, t21200: F, t21267: F, t26852: F, t3670: F, t6631: F, t6673: F, t6683: F, t97206: F, t21192: F, t104636: F, t104677: F, t104756: F, t104768: F, t104834: F, t1797: F, t20825: F, t20903: F, t20982: F, t20986: F, t29010: F, t5279: F, t7618: F, t1219: F, t30800: F, t1241: F, t21100: F, t7616: F, t1256: F, t30789: F, t104770: F, t1230: F, t20802: F, t21095: F, t21300: F, t21334: F, t2138: F, t29097: F, t30815: F, t484: F, t5261: F, t6619: F, t8184: F, t97177: F, t97250: F, t20786: F, t26849: F, t5265: F, t20819: F, t7617: F, t104696: F, t104793: F, t104815: F, t104817: F, t104825: F, t104828: F, t104833: F, t20797: F, t21046: F, t30799: F, t800: F, t21270: F, t2137: F, t467: F, t20926: F, t104647: F, t104844: F, t104924: F, t1227: F, t1791: F, t17934: F, t20838: F, t20923: F, t20934: F, t29062: F, t5320: F, t5343: F, t6611: F, t97292: F, t20850: F, t29086: F, t5362: F, t104863: F, t104872: F, t104916: F, t104946: F, t1238: F, t20858: F, t20952: F, t21042: F, t21310: F, t3767: F, t97179: F, t21169: F, t7607: F, t816: F, t8171: F, t104894: F, t104905: F, t20266: F, t20293: F, t20306: F, t20310: F, t20868: F, t21004: F, t21184: F, t29047: F, t29048: F, t29054: F, t29055: F, t57549: F, t6679: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t112175 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2240::<F>(t104624, t104626, t104640, t104651, t104653, t20806, t20811, t20876, t21153, t21166, t21259, t26870, t26880, t29100, t6690, t7624, t97182);
        let t112200 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2241::<F>(t6601, t7623, t21188, t26844, t104658, t104680, t104732, t1266, t17307, t1808, t20864, t20978, t21111, t26873, t29020, t29037, t5287, t5304, t5386, t6625, t7624, t97149);
        let (t112220, t112224) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2242::<F>(t104706, t4890, t104689, t104691, t104752, t104856, t104943, t20771, t20907, t20914, t20938, t20941, t20947, t26880, t29083, t3782, t5270, t5299, t5335, t6635, t7624, t97129, t97174);
        let t112249 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2243::<F>(t21233, t7624, t29083, t5378, t21090, t26867, t104703, t104708, t104715, t104774, t17183, t20959, t20963, t21030, t21246, t29040, t29096, t5335, t5348, t5354, t5397, t97141, t97261);
        let t112278 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2244::<F>(t29019, t5273, t20973, t7624, t1785, t29082, t104727, t104739, t104742, t1252, t1266, t1808, t21200, t21267, t26852, t29037, t29040, t3670, t5386, t5397, t6631, t6673, t6683, t97206);
        let t112299 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2245::<F>(t21192, t7624, t104636, t104677, t104756, t104768, t104834, t1797, t20825, t20903, t20982, t20986, t26880, t29010, t5279, t5287, t5299, t7618);
        let t112327 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2246::<F>(t1219, t30800, t1241, t21100, t7616, t1256, t30789, t104770, t1230, t1252, t20802, t21095, t21300, t21334, t2138, t26870, t29040, t29097, t30815, t484, t5261, t6619, t8184, t97177, t97250);
        let t112342 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2247::<F>(t29037, t5378, t20786, t26849, t29010, t5265, t20819, t7617, t104696, t104793, t104815, t104817, t104825, t104828, t104833, t1252, t20797, t21046, t97261);
        let t112372 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2248::<F>(t30799, t800, t21270, t2137, t467, t20926, t26870, t104647, t104752, t104844, t104924, t1227, t1266, t1791, t17934, t20838, t20923, t20934, t29062, t29096, t29100, t5279, t5320, t5343, t6611, t97174, t97292);
        let t112395 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2249::<F>(t20850, t2138, t29086, t5362, t104703, t104863, t104872, t104916, t104946, t112220, t1238, t1791, t20858, t20952, t21042, t21310, t26870, t29097, t3767, t5320, t5343, t5354, t97179);
        let (t112404, t112424) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2250::<F>(t21169, t7607, t816, t8171, t104894, t104905, t20266, t20293, t20306, t20310, t20868, t21004, t21184, t26852, t26880, t29047, t29048, t29054, t29055, t57549, t6679, t7624, t97179);
    (t112175, t112200, t112224, t112249, t112278, t112299, t112327, t112342, t112372, t112395, t112404, t112424)
}

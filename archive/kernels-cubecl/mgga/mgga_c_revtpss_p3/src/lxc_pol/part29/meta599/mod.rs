//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta599 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2039;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2040;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2041;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2042;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2043;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2044;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2045;
use chunk7::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2046;
use chunk8::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2047;
use chunk9::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2048;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta599<F: Float>(t101761: F, t103720: F, t103868: F, t118: F, t1310: F, t13426: F, t13514: F, t18163: F, t1843: F, t2014: F, t2089: F, t2322: F, t2372: F, t25177: F, t26210: F, t26396: F, t28586: F, t28653: F, t28683: F, t28711: F, t28737: F, t28750: F, t28926: F, t4151: F, t4254: F, t508: F, t5517: F, t651: F, t670: F, t7315: F, t7357: F, t7378: F, t7488: F, t7732: F, t7900: F, t7988: F, t8075: F, t8108: F, t95464: F, t98564: F, t102714: F, t10416: F, t13435: F, t1519: F, t18153: F, t1911: F, t2055: F, t2106: F, t2371: F, t25082: F, t26377: F, t26383: F, t26392: F, t26399: F, t26405: F, t26699: F, t27153: F, t28167: F, t28704: F, t28760: F, t33183: F, t3829: F, t4257: F, t7898: F, t7978: F, t7984: F, t8065: F, t95357: F, t98519: F, t101479: F, t102719: F, t13429: F, t14310: F, t1502: F, t2056: F, t2093: F, t2331: F, t25188: F, t26162: F, t26415: F, t26674: F, t28286: F, t28658: F, t3813: F, t4248: F, t49686: F, t5787: F, t73394: F, t73488: F, t7367: F, t7484: F, t75667: F, t7969: F, t8079: F, t8111: F, t98436: F, t13521: F, t13648: F, t1518: F, t18227: F, t2107: F, t26154: F, t26679: F, t27123: F, t27126: F, t27833: F, t28588: F, t28932: F, t28935: F, t49564: F, t7235: F, t7359: F, t7374: F, t7536: F, t7537: F, t75485: F, t95088: F, t97654: F, t98535: F, t2052: F, t2320: F, t25089: F, t26153: F, t26376: F, t26380: F, t26406: F, t28196: F, t28709: F, t28938: F, t5542: F, t649: F, t7489: F, t7539: F, t8109: F, t98450: F, t98550: F, t102009: F, t102058: F, t102791: F, t101725: F, t117: F, t1459: F, t18190: F, t18204: F, t18208: F, t18211: F, t1916: F, t2113: F, t2115: F, t26733: F, t26740: F, t28974: F, t28987: F, t28990: F, t4162: F, t4292: F, t572: F, t573: F, t5795: F, t5802: F, t5805: F, t7547: F, t7553: F, t7557: F, t8118: F, t96640: F, param_d: F, t28993: F, t571: F, t101724: F, t1458: F, t1464: F, t18178: F, t18217: F, t1921: F, t2111: F, t2118: F, t26704: F, t28945: F, t3: F, t4154: F, t4168: F, t575: F, t8114: F, t8130: F, t95182: F, t95184: F, t95186: F, t95190: F, t5789: F, t8113: F, t1913: F, t7560: F, t2110: F, t5808: F, t1455: F, t7541: F, t28944: F, t1456: F, t1914: F, t26743: F, t5790: F, t7542: F, t95196: F, t96633: F) -> F {
        let t103873 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2039::<F>(t101761, t103720, t103868, t118, t1310, t13426, t13514, t18163, t1843, t2014, t2089, t2322, t2372, t25177, t26210, t26396, t28586, t28653, t28683, t28711, t28737, t28750, t28926, t4151, t4254, t508, t5517, t651, t670, t7315, t7357, t7378, t7488, t7732, t7900, t7988, t8075, t8108, t95464, t98564);
        let t103917 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2040::<F>(t102714, t10416, t13435, t1519, t18153, t18163, t1911, t2014, t2055, t2106, t2322, t2371, t25082, t26377, t26383, t26392, t26399, t26405, t26699, t27153, t28167, t28704, t28750, t28760, t33183, t3829, t4254, t4257, t651, t7898, t7900, t7978, t7984, t7988, t8065, t95357, t98519);
        let t103956 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2041::<F>(t101479, t102719, t13426, t13429, t14310, t1502, t1519, t2056, t2089, t2093, t2331, t25082, t25188, t26162, t26405, t26415, t26674, t28167, t28286, t28653, t28658, t3813, t4248, t4257, t49686, t5787, t73394, t73488, t7367, t7484, t75667, t7898, t7969, t8079, t8111, t98436);
        let t103999 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2042::<F>(t10416, t13435, t13521, t13648, t1518, t18227, t2014, t2056, t2107, t2322, t25082, t26154, t26674, t26679, t27123, t27126, t27833, t28286, t28588, t28760, t28932, t28935, t49564, t651, t7235, t7359, t7367, t7374, t7536, t7537, t75485, t7732, t7898, t7978, t95088, t97654, t98535);
        let t104038 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2043::<F>(t10416, t13426, t13435, t18153, t18227, t1843, t2014, t2052, t2320, t2322, t25089, t25188, t26153, t26376, t26380, t26396, t26406, t27833, t28196, t28286, t28586, t28704, t28709, t28938, t4248, t5542, t649, t651, t7235, t7374, t7489, t7539, t7898, t7984, t8065, t8109, t98450, t98550);
        let (t104041, t104054) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2044::<F>(t102009, t102058, t102791, t103873, t103917, t103956, t103999, t104038, t101725, t101761, t117, t13514, t1459, t1518, t18190, t18204, t18208, t18211, t1916, t2113, t2115, t26733, t26740, t28974, t28987, t28990, t4162, t4292, t572, t573, t5795, t5802, t5805, t7547, t7553, t7557, t8118, t96640, param_d);
        let t104065 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2045::<F>(t28993, t571, t101724, t104041, t104054, t1458, t1464, t18178, t18217, t1921, t2111, t2118, t26704, t28945, t3, t4154, t4168, t575, t8114, t8130, t95182, t95184, t95186, t95190);
        let (t104071, t104073, t104077, t104079, t104081, t104083) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2046::<F>(t2118, t5789, t1464, t8113, t1913, t7560, t2110, t5808, t1455, t8130, t1921, t7541);
        let t104087 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2047::<F>(t28944, t575, t104071, t104073, t104077, t104079, t104081, t104083, t1456, t1914, t26743, t28993, t5790, t5808, t7542, t7560, t95196, t96633);
        let tv4rho3sigma4 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2048::<F>(t104065, t104087);
    tv4rho3sigma4
}

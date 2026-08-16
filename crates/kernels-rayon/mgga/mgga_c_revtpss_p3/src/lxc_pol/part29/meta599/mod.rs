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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta599(t101761: f64, t103720: f64, t103868: f64, t118: f64, t1310: f64, t13426: f64, t13514: f64, t18163: f64, t1843: f64, t2014: f64, t2089: f64, t2322: f64, t2372: f64, t25177: f64, t26210: f64, t26396: f64, t28586: f64, t28653: f64, t28683: f64, t28711: f64, t28737: f64, t28750: f64, t28926: f64, t4151: f64, t4254: f64, t508: f64, t5517: f64, t651: f64, t670: f64, t7315: f64, t7357: f64, t7378: f64, t7488: f64, t7732: f64, t7900: f64, t7988: f64, t8075: f64, t8108: f64, t95464: f64, t98564: f64, t102714: f64, t10416: f64, t13435: f64, t1519: f64, t18153: f64, t1911: f64, t2055: f64, t2106: f64, t2371: f64, t25082: f64, t26377: f64, t26383: f64, t26392: f64, t26399: f64, t26405: f64, t26699: f64, t27153: f64, t28167: f64, t28704: f64, t28760: f64, t33183: f64, t3829: f64, t4257: f64, t7898: f64, t7978: f64, t7984: f64, t8065: f64, t95357: f64, t98519: f64, t101479: f64, t102719: f64, t13429: f64, t14310: f64, t1502: f64, t2056: f64, t2093: f64, t2331: f64, t25188: f64, t26162: f64, t26415: f64, t26674: f64, t28286: f64, t28658: f64, t3813: f64, t4248: f64, t49686: f64, t5787: f64, t73394: f64, t73488: f64, t7367: f64, t7484: f64, t75667: f64, t7969: f64, t8079: f64, t8111: f64, t98436: f64, t13521: f64, t13648: f64, t1518: f64, t18227: f64, t2107: f64, t26154: f64, t26679: f64, t27123: f64, t27126: f64, t27833: f64, t28588: f64, t28932: f64, t28935: f64, t49564: f64, t7235: f64, t7359: f64, t7374: f64, t7536: f64, t7537: f64, t75485: f64, t95088: f64, t97654: f64, t98535: f64, t2052: f64, t2320: f64, t25089: f64, t26153: f64, t26376: f64, t26380: f64, t26406: f64, t28196: f64, t28709: f64, t28938: f64, t5542: f64, t649: f64, t7489: f64, t7539: f64, t8109: f64, t98450: f64, t98550: f64, t102009: f64, t102058: f64, t102791: f64, t101725: f64, t117: f64, t1459: f64, t18190: f64, t18204: f64, t18208: f64, t18211: f64, t1916: f64, t2113: f64, t2115: f64, t26733: f64, t26740: f64, t28974: f64, t28987: f64, t28990: f64, t4162: f64, t4292: f64, t572: f64, t573: f64, t5795: f64, t5802: f64, t5805: f64, t7547: f64, t7553: f64, t7557: f64, t8118: f64, t96640: f64, param_d: f64, t28993: f64, t571: f64, t101724: f64, t1458: f64, t1464: f64, t18178: f64, t18217: f64, t1921: f64, t2111: f64, t2118: f64, t26704: f64, t28945: f64, t3: f64, t4154: f64, t4168: f64, t575: f64, t8114: f64, t8130: f64, t95182: f64, t95184: f64, t95186: f64, t95190: f64, t5789: f64, t8113: f64, t1913: f64, t7560: f64, t2110: f64, t5808: f64, t1455: f64, t7541: f64, t28944: f64, t1456: f64, t1914: f64, t26743: f64, t5790: f64, t7542: f64, t95196: f64, t96633: f64) -> f64 {
        let t103873 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2039(t101761, t103720, t103868, t118, t1310, t13426, t13514, t18163, t1843, t2014, t2089, t2322, t2372, t25177, t26210, t26396, t28586, t28653, t28683, t28711, t28737, t28750, t28926, t4151, t4254, t508, t5517, t651, t670, t7315, t7357, t7378, t7488, t7732, t7900, t7988, t8075, t8108, t95464, t98564);
        let t103917 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2040(t102714, t10416, t13435, t1519, t18153, t18163, t1911, t2014, t2055, t2106, t2322, t2371, t25082, t26377, t26383, t26392, t26399, t26405, t26699, t27153, t28167, t28704, t28750, t28760, t33183, t3829, t4254, t4257, t651, t7898, t7900, t7978, t7984, t7988, t8065, t95357, t98519);
        let t103956 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2041(t101479, t102719, t13426, t13429, t14310, t1502, t1519, t2056, t2089, t2093, t2331, t25082, t25188, t26162, t26405, t26415, t26674, t28167, t28286, t28653, t28658, t3813, t4248, t4257, t49686, t5787, t73394, t73488, t7367, t7484, t75667, t7898, t7969, t8079, t8111, t98436);
        let t103999 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2042(t10416, t13435, t13521, t13648, t1518, t18227, t2014, t2056, t2107, t2322, t25082, t26154, t26674, t26679, t27123, t27126, t27833, t28286, t28588, t28760, t28932, t28935, t49564, t651, t7235, t7359, t7367, t7374, t7536, t7537, t75485, t7732, t7898, t7978, t95088, t97654, t98535);
        let t104038 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2043(t10416, t13426, t13435, t18153, t18227, t1843, t2014, t2052, t2320, t2322, t25089, t25188, t26153, t26376, t26380, t26396, t26406, t27833, t28196, t28286, t28586, t28704, t28709, t28938, t4248, t5542, t649, t651, t7235, t7374, t7489, t7539, t7898, t7984, t8065, t8109, t98450, t98550);
        let (t104041, t104054) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2044(t102009, t102058, t102791, t103873, t103917, t103956, t103999, t104038, t101725, t101761, t117, t13514, t1459, t1518, t18190, t18204, t18208, t18211, t1916, t2113, t2115, t26733, t26740, t28974, t28987, t28990, t4162, t4292, t572, t573, t5795, t5802, t5805, t7547, t7553, t7557, t8118, t96640, param_d);
        let t104065 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2045(t28993, t571, t101724, t104041, t104054, t1458, t1464, t18178, t18217, t1921, t2111, t2118, t26704, t28945, t3, t4154, t4168, t575, t8114, t8130, t95182, t95184, t95186, t95190);
        let (t104071, t104073, t104077, t104079, t104081, t104083) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2046(t2118, t5789, t1464, t8113, t1913, t7560, t2110, t5808, t1455, t8130, t1921, t7541);
        let t104087 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2047(t28944, t575, t104071, t104073, t104077, t104079, t104081, t104083, t1456, t1914, t26743, t28993, t5790, t5808, t7542, t7560, t95196, t96633);
        let tv4rho3sigma4 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2048(t104065, t104087);
    tv4rho3sigma4
}

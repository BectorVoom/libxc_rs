//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta456 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1649;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1650;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1651;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1652;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1653;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1654;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1655;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1656;
use chunk8::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1657;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta456(t1248: f64, t3604: f64, t6688: f64, t3720: f64, t20266: f64, t5312: f64, t17475: f64, t20293: f64, t20318: f64, t5308: f64, t20310: f64, t20306: f64, t1260: f64, t6601: f64, t1222: f64, t1266: f64, t12784: f64, t12855: f64, t17437: f64, t5304: f64, t5309: f64, t5313: f64, t5373: f64, t5391: f64, t6640: f64, t1264: f64, t20272: f64, t247: f64, t5405: f64, t6429: f64, t3626: f64, t6425: f64, t1794: f64, t5245: f64, t1250: f64, t140: f64, t6652: f64, t20795: f64, t3629: f64, t1261: f64, t17412: f64, t17444: f64, t17447: f64, t17453: f64, t17474: f64, t1808: f64, t3625: f64, t3647: f64, t3718: f64, t5331: f64, t6673: f64, t1234: f64, t6594: f64, t1214: f64, t5825: f64, t5296: f64, t1042: f64, t3172: f64, t6630: f64, t3600: f64, t3634: f64, t1238: f64, t12882: f64, t12893: f64, t12900: f64, t12905: f64, t12985: f64, t17509: f64, t17546: f64, t17556: f64, t3711: f64, t20721: f64, t3719: f64, t3670: f64, t5390: f64, t1225: f64, t18281: f64, t1012: f64, t1010: f64, t5843: f64, t5378: f64, t5381: f64, t21040: f64, t12840: f64, t1227: f64, t13012: f64, t17593: f64, t17619: f64, t17622: f64, t5340: f64, t5369: f64, t5384: f64, t5386: f64, t17633: f64, t6638: f64, t12884: f64, t6421: f64, t20302: f64, t20298: f64, t1785: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21121, t21126, t21129, t21134, t21137, t21140) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1649(t1248, t3604, t6688, t3720, t20266, t5312, t17475, t20293, t20318, t5308, t20310, t20306);
        let t21146 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1650(t1260, t6601, t1222, t1266, t12784, t12855, t17437, t21121, t21126, t21129, t21134, t21137, t21140, t5304, t5309, t5313, t5373, t5391, t6640);
        let (t21153, t21157, t21161, t21164, t21166, t21169) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1651(t1264, t20272, t247, t5405, t6429, t3626, t6425, t1794, t5245, t1250, t3720, t140, t6652);
        let t21176 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1652(t1222, t21169, t20795, t3629, t3626, t1261, t17412, t17444, t17447, t17453, t17474, t1808, t21153, t21157, t21161, t21166, t3625, t3647, t3718, t5331, t6673);
        let (t21177, t21184, t21189, t21192) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1653(t1234, t6594, t1214, t5825, t5296, t1042, t3172, t6630, t3600, t247, t3634, t6425);
        let t21196 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1654(t1261, t21192, t1238, t12882, t12893, t12900, t12905, t12985, t17509, t17546, t17556, t21177, t21184, t21189, t3711);
        let (t21200, t21203, t21210, t21213, t21216) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1655(t20721, t247, t3719, t3670, t5390, t1225, t18281, t1012, t1010, t5843, t5378, t5381);
        let t21226 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1656(t21040, t3629, t3626, t12840, t20795, t1222, t1227, t13012, t17593, t17619, t17622, t21200, t21203, t21210, t21213, t21216, t3625, t5340, t5369, t5373, t5384, t5386);
        let (t21228, t21234, t21236, t21239, t21242) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1657(t17633, t6638, t3626, t12884, t247, t6421, t1261, t20302, t5312, t20298, t1785, t5390);
    (t21146, t21164, t21176, t21196, t21226, t21228, t21234, t21236, t21239, t21242)
}

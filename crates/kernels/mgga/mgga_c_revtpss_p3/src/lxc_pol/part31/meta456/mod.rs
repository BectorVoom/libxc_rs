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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta456<F: Float>(t1248: F, t3604: F, t6688: F, t3720: F, t20266: F, t5312: F, t17475: F, t20293: F, t20318: F, t5308: F, t20310: F, t20306: F, t1260: F, t6601: F, t1222: F, t1266: F, t12784: F, t12855: F, t17437: F, t5304: F, t5309: F, t5313: F, t5373: F, t5391: F, t6640: F, t1264: F, t20272: F, t247: F, t5405: F, t6429: F, t3626: F, t6425: F, t1794: F, t5245: F, t1250: F, t140: F, t6652: F, t20795: F, t3629: F, t1261: F, t17412: F, t17444: F, t17447: F, t17453: F, t17474: F, t1808: F, t3625: F, t3647: F, t3718: F, t5331: F, t6673: F, t1234: F, t6594: F, t1214: F, t5825: F, t5296: F, t1042: F, t3172: F, t6630: F, t3600: F, t3634: F, t1238: F, t12882: F, t12893: F, t12900: F, t12905: F, t12985: F, t17509: F, t17546: F, t17556: F, t3711: F, t20721: F, t3719: F, t3670: F, t5390: F, t1225: F, t18281: F, t1012: F, t1010: F, t5843: F, t5378: F, t5381: F, t21040: F, t12840: F, t1227: F, t13012: F, t17593: F, t17619: F, t17622: F, t5340: F, t5369: F, t5384: F, t5386: F, t17633: F, t6638: F, t12884: F, t6421: F, t20302: F, t20298: F, t1785: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t21121, t21126, t21129, t21134, t21137, t21140) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1649::<F>(t1248, t3604, t6688, t3720, t20266, t5312, t17475, t20293, t20318, t5308, t20310, t20306);
        let t21146 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1650::<F>(t1260, t6601, t1222, t1266, t12784, t12855, t17437, t21121, t21126, t21129, t21134, t21137, t21140, t5304, t5309, t5313, t5373, t5391, t6640);
        let (t21153, t21157, t21161, t21164, t21166, t21169) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1651::<F>(t1264, t20272, t247, t5405, t6429, t3626, t6425, t1794, t5245, t1250, t3720, t140, t6652);
        let t21176 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1652::<F>(t1222, t21169, t20795, t3629, t3626, t1261, t17412, t17444, t17447, t17453, t17474, t1808, t21153, t21157, t21161, t21166, t3625, t3647, t3718, t5331, t6673);
        let (t21177, t21184, t21189, t21192) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1653::<F>(t1234, t6594, t1214, t5825, t5296, t1042, t3172, t6630, t3600, t247, t3634, t6425);
        let t21196 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1654::<F>(t1261, t21192, t1238, t12882, t12893, t12900, t12905, t12985, t17509, t17546, t17556, t21177, t21184, t21189, t3711);
        let (t21200, t21203, t21210, t21213, t21216) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1655::<F>(t20721, t247, t3719, t3670, t5390, t1225, t18281, t1012, t1010, t5843, t5378, t5381);
        let t21226 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1656::<F>(t21040, t3629, t3626, t12840, t20795, t1222, t1227, t13012, t17593, t17619, t17622, t21200, t21203, t21210, t21213, t21216, t3625, t5340, t5369, t5373, t5384, t5386);
        let (t21228, t21234, t21236, t21239, t21242) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1657::<F>(t17633, t6638, t3626, t12884, t247, t6421, t1261, t20302, t5312, t20298, t1785, t5390);
    (t21146, t21164, t21176, t21196, t21226, t21228, t21234, t21236, t21239, t21242)
}

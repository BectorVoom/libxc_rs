//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta455 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1654;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1655;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1656;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1657;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta455(t21082: f64, t482: f64, t371: f64, t372: f64, t5323: f64, t5362: f64, t12772: f64, t6639: f64, t3625: f64, t1263: f64, t6573: f64, t1122: f64, t1042: f64, t1038: f64, t6593: f64, t1244: f64, t1241: f64, t5273: f64, t5292: f64, t17235: f64, t19661: f64, t1235: f64, t1238: f64, t1252: f64, t1261: f64, t17505: f64, t17569: f64, t21063: f64, t3667: f64, t5279: f64, t5320: f64, t5327: f64, t5384: f64, t6647: f64, t1248: f64, t3604: f64, t6688: f64, t3720: f64, t20266: f64, t5312: f64, t17475: f64, t20293: f64, t20318: f64, t5308: f64, t20310: f64, t20306: f64, t1260: f64, t6601: f64, t1222: f64, t1266: f64, t12784: f64, t12855: f64, t17437: f64, t5304: f64, t5309: f64, t5313: f64, t5373: f64, t5391: f64, t6640: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21085, t21088, t21090, t21091, t21094) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1654(t21082, t482, t371, t372, t5323, t5362, t12772, t6639, t3625, t1263, t6573, t1122);
        let (t21095, t21100, t21111, t21114) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1655(t1042, t21094, t1038, t6593, t1244, t1241, t5273, t5292, t17235, t19661, t1235, t1238, t1252, t1261, t17505, t17569, t21063, t21085, t21088, t21091, t3667, t5279, t5320, t5327, t5384, t6647);
        let (t21121, t21126, t21129, t21134, t21137, t21140) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1656(t1248, t3604, t6688, t3720, t20266, t5312, t17475, t20293, t20318, t5308, t20310, t20306);
        let t21146 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1657(t1260, t6601, t1222, t1266, t12784, t12855, t17437, t21121, t21126, t21129, t21134, t21137, t21140, t5304, t5309, t5313, t5373, t5391, t6640);
    (t21085, t21090, t21095, t21100, t21111, t21114, t21121, t21146)
}

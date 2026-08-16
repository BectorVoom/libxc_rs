//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta455 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1654;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1655;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1656;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1657;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta455<F: Float>(t21082: F, t482: F, t371: F, t372: F, t5323: F, t5362: F, t12772: F, t6639: F, t3625: F, t1263: F, t6573: F, t1122: F, t1042: F, t1038: F, t6593: F, t1244: F, t1241: F, t5273: F, t5292: F, t17235: F, t19661: F, t1235: F, t1238: F, t1252: F, t1261: F, t17505: F, t17569: F, t21063: F, t3667: F, t5279: F, t5320: F, t5327: F, t5384: F, t6647: F, t1248: F, t3604: F, t6688: F, t3720: F, t20266: F, t5312: F, t17475: F, t20293: F, t20318: F, t5308: F, t20310: F, t20306: F, t1260: F, t6601: F, t1222: F, t1266: F, t12784: F, t12855: F, t17437: F, t5304: F, t5309: F, t5313: F, t5373: F, t5391: F, t6640: F) -> (F, F, F, F, F, F, F, F) {
        let (t21085, t21088, t21090, t21091, t21094) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1654::<F>(t21082, t482, t371, t372, t5323, t5362, t12772, t6639, t3625, t1263, t6573, t1122);
        let (t21095, t21100, t21111, t21114) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1655::<F>(t1042, t21094, t1038, t6593, t1244, t1241, t5273, t5292, t17235, t19661, t1235, t1238, t1252, t1261, t17505, t17569, t21063, t21085, t21088, t21091, t3667, t5279, t5320, t5327, t5384, t6647);
        let (t21121, t21126, t21129, t21134, t21137, t21140) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1656::<F>(t1248, t3604, t6688, t3720, t20266, t5312, t17475, t20293, t20318, t5308, t20310, t20306);
        let t21146 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1657::<F>(t1260, t6601, t1222, t1266, t12784, t12855, t17437, t21121, t21126, t21129, t21134, t21137, t21140, t5304, t5309, t5313, t5373, t5391, t6640);
    (t21085, t21090, t21095, t21100, t21111, t21114, t21121, t21146)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta680 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2214;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2215;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2216;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2217;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2218;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2219;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2220;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta680(t2042: f64, t22544: f64, t26123: f64, t572: f64, t5920: f64, t5883: f64, t7002: f64, t101622: f64, t1518: f64, t28276: f64, t4292: f64, t30974: f64, t575: f64, t2121: f64, t2247: f64, t5819: f64, t1469: f64, t603: f64, t108737: f64, t108745: f64, t108749: f64, t108759: f64, t108762: f64, t108765: f64, t108816: f64, t2123: f64, t26749: f64, t26755: f64, t29375: f64, t29548: f64, t29554: f64, t6960: f64, t7566: f64, t7576: f64, t7709: f64, t104181: f64, t104185: f64, t28105: f64, t28109: f64, t28112: f64, t28116: f64, t28119: f64, t29364: f64, t29367: f64, t29412: f64, t29538: f64, t7579: f64, t7706: f64, t8144: f64, t30681: f64, t38: f64, t108733: f64, t28133: f64, t28141: f64, t29372: f64, t29388: f64, t29544: f64, t30683: f64, t6963: f64, t8147: f64, t60673: f64, t7565: f64, t13272: f64, t29411: f64, t104279: f64, t104282: f64, t108769: f64, t108792: f64, t108864: f64, t26792: f64, t29562: f64, t30686: f64, t30689: f64, t96824: f64, t96827: f64, t10309: f64, t104317: f64, t108807: f64, t108810: f64, t108813: f64, t1470: f64, t28093: f64, t28147: f64, t28154: f64, t29513: f64, t29551: f64, t5842: f64, t60: f64, t104379: f64, t108952: f64, t18281: f64, t1923: f64, t1927: f64, t19661: f64, t19666: f64, t19680: f64, t26776: f64, t28089: f64, t29355: f64, t29363: f64, t30682: f64, t4181: f64, t4186: f64, t606: f64, t6954: f64, t6977: f64, t72: f64, t7571: f64, t7702: f64, t7719: f64, t8143: f64, t92612: f64, t96733: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t109319, t109322, t109327, t109330, t109333, t111419) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2214(t2042, t22544, t26123, t572, t5920, t5883, t7002, t101622, t1518, t28276, t4292, t30974, t575);
        let t111468 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2215(t2121, t2247, t5819, t1469, t603, t108737, t108745, t108749, t108759, t108762, t108765, t108816, t2123, t26749, t26755, t29375, t29548, t29554, t6960, t7566, t7576, t7709);
        let t111493 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2216(t104181, t104185, t28105, t28109, t28112, t28116, t28119, t29364, t29367, t29412, t29538, t29554, t7576, t7579, t7706, t7709, t8144);
        let t111521 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2217(t2247, t30681, t38, t108733, t26749, t26755, t28112, t28116, t28119, t28133, t28141, t29372, t29388, t29544, t30683, t6960, t6963, t7566, t7709, t8144, t8147);
        let t111548 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2218(t60673, t7565, t13272, t29411, t104279, t104282, t108769, t108792, t108864, t2123, t26792, t28133, t29412, t29562, t30686, t30689, t6960, t6963, t7566, t7706, t96824, t96827);
        let t111577 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2219(t10309, t104317, t108807, t108810, t108813, t1470, t2121, t2123, t28093, t28105, t28109, t28147, t28154, t29388, t29513, t29551, t7576, t7579, t8144);
        let t111623 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2220(t5842, t60, t104379, t108952, t18281, t1923, t1927, t19661, t19666, t19680, t2123, t26776, t28089, t28093, t29355, t29363, t29364, t29367, t29372, t29375, t30682, t30683, t30686, t4181, t4186, t606, t6954, t6977, t72, t7571, t7702, t7719, t8143, t8147, t92612, t96733);
    (t109319, t109322, t109327, t109330, t109333, t111419, t111468, t111493, t111521, t111548, t111577, t111623)
}

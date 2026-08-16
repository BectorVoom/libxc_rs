//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta680 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2214;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2215;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2216;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2217;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2218;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2219;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2220;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta680<F: Float>(t2042: F, t22544: F, t26123: F, t572: F, t5920: F, t5883: F, t7002: F, t101622: F, t1518: F, t28276: F, t4292: F, t30974: F, t575: F, t2121: F, t2247: F, t5819: F, t1469: F, t603: F, t108737: F, t108745: F, t108749: F, t108759: F, t108762: F, t108765: F, t108816: F, t2123: F, t26749: F, t26755: F, t29375: F, t29548: F, t29554: F, t6960: F, t7566: F, t7576: F, t7709: F, t104181: F, t104185: F, t28105: F, t28109: F, t28112: F, t28116: F, t28119: F, t29364: F, t29367: F, t29412: F, t29538: F, t7579: F, t7706: F, t8144: F, t30681: F, t38: F, t108733: F, t28133: F, t28141: F, t29372: F, t29388: F, t29544: F, t30683: F, t6963: F, t8147: F, t60673: F, t7565: F, t13272: F, t29411: F, t104279: F, t104282: F, t108769: F, t108792: F, t108864: F, t26792: F, t29562: F, t30686: F, t30689: F, t96824: F, t96827: F, t10309: F, t104317: F, t108807: F, t108810: F, t108813: F, t1470: F, t28093: F, t28147: F, t28154: F, t29513: F, t29551: F, t5842: F, t60: F, t104379: F, t108952: F, t18281: F, t1923: F, t1927: F, t19661: F, t19666: F, t19680: F, t26776: F, t28089: F, t29355: F, t29363: F, t30682: F, t4181: F, t4186: F, t606: F, t6954: F, t6977: F, t72: F, t7571: F, t7702: F, t7719: F, t8143: F, t92612: F, t96733: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t109319, t109322, t109327, t109330, t109333, t111419) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2214::<F>(t2042, t22544, t26123, t572, t5920, t5883, t7002, t101622, t1518, t28276, t4292, t30974, t575);
        let t111468 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2215::<F>(t2121, t2247, t5819, t1469, t603, t108737, t108745, t108749, t108759, t108762, t108765, t108816, t2123, t26749, t26755, t29375, t29548, t29554, t6960, t7566, t7576, t7709);
        let t111493 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2216::<F>(t104181, t104185, t28105, t28109, t28112, t28116, t28119, t29364, t29367, t29412, t29538, t29554, t7576, t7579, t7706, t7709, t8144);
        let t111521 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2217::<F>(t2247, t30681, t38, t108733, t26749, t26755, t28112, t28116, t28119, t28133, t28141, t29372, t29388, t29544, t30683, t6960, t6963, t7566, t7709, t8144, t8147);
        let t111548 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2218::<F>(t60673, t7565, t13272, t29411, t104279, t104282, t108769, t108792, t108864, t2123, t26792, t28133, t29412, t29562, t30686, t30689, t6960, t6963, t7566, t7706, t96824, t96827);
        let t111577 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2219::<F>(t10309, t104317, t108807, t108810, t108813, t1470, t2121, t2123, t28093, t28105, t28109, t28147, t28154, t29388, t29513, t29551, t7576, t7579, t8144);
        let t111623 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2220::<F>(t5842, t60, t104379, t108952, t18281, t1923, t1927, t19661, t19666, t19680, t2123, t26776, t28089, t28093, t29355, t29363, t29364, t29367, t29372, t29375, t30682, t30683, t30686, t4181, t4186, t606, t6954, t6977, t72, t7571, t7702, t7719, t8143, t8147, t92612, t96733);
    (t109319, t109322, t109327, t109330, t109333, t111419, t111468, t111493, t111521, t111548, t111577, t111623)
}

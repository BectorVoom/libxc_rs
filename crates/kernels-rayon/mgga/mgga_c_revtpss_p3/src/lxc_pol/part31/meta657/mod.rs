//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta657 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2214;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2215;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2216;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2217;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2218;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2219;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2220;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2221;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta657(t27932: f64, t74477: f64, t74419: f64, t98196: f64, t74423: f64, t22021: f64, t25986: f64, t2661: f64, t22068: f64, t25972: f64, t25978: f64, t6880: f64, t6856: f64, t102569: f64, t94554: f64, t94565: f64, t94569: f64, t94571: f64, t98282: f64, t108530: f64, t108551: f64, t108564: f64, t108580: f64, t108589: f64, t108596: f64, t108613: f64, t108502: f64, t14230: f64, t1903: f64, t213: f64, t22395: f64, t225: f64, t25930: f64, t25931: f64, t27868: f64, t27980: f64, t561: f64, t7279: f64, t75016: f64, t94884: f64, t98333: f64, t98338: f64, t98358: f64, t98360: f64, t98368: f64, t98372: f64, t98376: f64, t98379: f64, t1398: f64, t543: f64, t6895: f64, t1904: f64, t27985: f64, t689: f64, t108484: f64, t2027: f64, t2028: f64, t25921: f64, t26079: f64, t26084: f64, t30082: f64, t4003: f64, t545: f64, t6919: f64, t7295: f64, t94823: f64, t94914: f64, t94917: f64, t94919: f64, t94931: f64, t98382: f64, t98384: f64, t98387: f64, t98390: f64, t98399: f64, t108145: f64, t108172: f64, t108213: f64, t108233: f64, t108270: f64, t108310: f64, t108327: f64, t108349: f64, t108374: f64, t108399: f64, t108425: f64, t108443: f64, t108471: f64, t108500: f64, t1450: f64, t2014: f64, t532: f64, t1907: f64, t5591: f64, t25082: f64, t8717: f64, t29495: f64, t7235: f64, t5778: f64, t28196: f64, t28197: f64, t28184: f64, t7898: f64, t5920: f64, t648: f64, t1937: f64, t108117: f64, t108120: f64, t108129: f64, t1453: f64, t1502: f64, t1519: f64, t2007: f64, t21881: f64, t21882: f64, t27830: f64, t28030: f64, t28050: f64, t29986: f64, t30150: f64, t4246: f64, t4248: f64, t4257: f64, t4293: f64, t651: f64, t670: f64, t6985: f64, t7883: f64, t97622: f64, t94: f64, t29508: f64, t6993: f64, t86815: f64, t7003: f64, t27123: f64, t7735: f64, t27126: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t108615, t108617, t108619, t108623, t108625, t108627) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2214(t27932, t74477, t74419, t98196, t74423, t22021, t25986, t2661, t22068, t25972, t25978, t6880);
        let t108631 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2215(t25978, t6856, t102569, t108615, t108617, t108619, t108623, t108625, t108627, t94554, t94565, t94569, t94571, t98282);
        let (t108634, t108651) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2216(t108530, t108551, t108564, t108580, t108589, t108596, t108613, t108631, t108502, t14230, t1903, t213, t22395, t225, t25930, t25931, t27868, t27980, t561, t7279, t75016, t94884, t98333, t98338, t98358, t98360, t98368, t98372, t98376, t98379);
        let t108674 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2217(t1398, t543, t6895, t1904, t27985, t689, t108484, t108634, t2027, t2028, t25921, t25931, t26079, t26084, t30082, t4003, t545, t6919, t7295, t94823, t94914, t94917, t94919, t94931, t98382, t98384, t98387, t98390, t98399);
        let t108681 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2218(t108145, t108172, t108213, t108233, t108270, t108310, t108327, t108349, t108374, t108399, t108425, t108443, t108471, t108500, t108651, t108674, t1450, t2014, t532);
        let (t108685, t108687, t108691, t108693, t108710) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2219(t1907, t5591, t25082, t8717, t29495, t7235, t5778, t28196, t28197, t28184, t7898, t5920, t648);
        let t108713 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2220(t108710, t1937, t108117, t108120, t108129, t108681, t108685, t108687, t108691, t108693, t1453, t1502, t1519, t2007, t21881, t21882, t27830, t28030, t28050, t29986, t30150, t4246, t4248, t4257, t4293, t651, t670, t6985, t7883, t97622);
        let (t108716, t108718, t108721, t108723, t108725, t108727) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2221(t21881, t94, t1937, t29508, t6993, t25082, t86815, t8717, t7003, t27123, t7735, t27126);
    (t108710, t108713, t108716, t108718, t108721, t108723, t108725, t108727)
}

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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2214;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2215;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2216;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2217;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2218;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2219;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2220;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2221;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta657<F: Float>(t27932: F, t74477: F, t74419: F, t98196: F, t74423: F, t22021: F, t25986: F, t2661: F, t22068: F, t25972: F, t25978: F, t6880: F, t6856: F, t102569: F, t94554: F, t94565: F, t94569: F, t94571: F, t98282: F, t108530: F, t108551: F, t108564: F, t108580: F, t108589: F, t108596: F, t108613: F, t108502: F, t14230: F, t1903: F, t213: F, t22395: F, t225: F, t25930: F, t25931: F, t27868: F, t27980: F, t561: F, t7279: F, t75016: F, t94884: F, t98333: F, t98338: F, t98358: F, t98360: F, t98368: F, t98372: F, t98376: F, t98379: F, t1398: F, t543: F, t6895: F, t1904: F, t27985: F, t689: F, t108484: F, t2027: F, t2028: F, t25921: F, t26079: F, t26084: F, t30082: F, t4003: F, t545: F, t6919: F, t7295: F, t94823: F, t94914: F, t94917: F, t94919: F, t94931: F, t98382: F, t98384: F, t98387: F, t98390: F, t98399: F, t108145: F, t108172: F, t108213: F, t108233: F, t108270: F, t108310: F, t108327: F, t108349: F, t108374: F, t108399: F, t108425: F, t108443: F, t108471: F, t108500: F, t1450: F, t2014: F, t532: F, t1907: F, t5591: F, t25082: F, t8717: F, t29495: F, t7235: F, t5778: F, t28196: F, t28197: F, t28184: F, t7898: F, t5920: F, t648: F, t1937: F, t108117: F, t108120: F, t108129: F, t1453: F, t1502: F, t1519: F, t2007: F, t21881: F, t21882: F, t27830: F, t28030: F, t28050: F, t29986: F, t30150: F, t4246: F, t4248: F, t4257: F, t4293: F, t651: F, t670: F, t6985: F, t7883: F, t97622: F, t94: F, t29508: F, t6993: F, t86815: F, t7003: F, t27123: F, t7735: F, t27126: F) -> (F, F, F, F, F, F, F, F) {
        let (t108615, t108617, t108619, t108623, t108625, t108627) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2214::<F>(t27932, t74477, t74419, t98196, t74423, t22021, t25986, t2661, t22068, t25972, t25978, t6880);
        let t108631 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2215::<F>(t25978, t6856, t102569, t108615, t108617, t108619, t108623, t108625, t108627, t94554, t94565, t94569, t94571, t98282);
        let (t108634, t108651) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2216::<F>(t108530, t108551, t108564, t108580, t108589, t108596, t108613, t108631, t108502, t14230, t1903, t213, t22395, t225, t25930, t25931, t27868, t27980, t561, t7279, t75016, t94884, t98333, t98338, t98358, t98360, t98368, t98372, t98376, t98379);
        let t108674 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2217::<F>(t1398, t543, t6895, t1904, t27985, t689, t108484, t108634, t2027, t2028, t25921, t25931, t26079, t26084, t30082, t4003, t545, t6919, t7295, t94823, t94914, t94917, t94919, t94931, t98382, t98384, t98387, t98390, t98399);
        let t108681 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2218::<F>(t108145, t108172, t108213, t108233, t108270, t108310, t108327, t108349, t108374, t108399, t108425, t108443, t108471, t108500, t108651, t108674, t1450, t2014, t532);
        let (t108685, t108687, t108691, t108693, t108710) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2219::<F>(t1907, t5591, t25082, t8717, t29495, t7235, t5778, t28196, t28197, t28184, t7898, t5920, t648);
        let t108713 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2220::<F>(t108710, t1937, t108117, t108120, t108129, t108681, t108685, t108687, t108691, t108693, t1453, t1502, t1519, t2007, t21881, t21882, t27830, t28030, t28050, t29986, t30150, t4246, t4248, t4257, t4293, t651, t670, t6985, t7883, t97622);
        let (t108716, t108718, t108721, t108723, t108725, t108727) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2221::<F>(t21881, t94, t1937, t29508, t6993, t25082, t86815, t8717, t7003, t27123, t7735, t27126);
    (t108710, t108713, t108716, t108718, t108721, t108723, t108725, t108727)
}

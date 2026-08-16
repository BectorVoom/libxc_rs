//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta666 (260520-c91 hierarchical CSE).
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
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2179;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2180;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2181;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2182;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2183;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2184;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2185;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2186;
use chunk8::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2187;
use chunk9::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2188;
use chunk10::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2189;
use chunk11::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2190;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta666<F: Float>(t22048: F, t27940: F, t22089: F, t22146: F, t26004: F, t6884: F, t6850: F, t94513: F, t22041: F, t7252: F, t22295: F, t26028: F, t22299: F, t22093: F, t22098: F, t98129: F, t98131: F, t2018: F, t22129: F, t807: F, t22262: F, t25986: F, t2661: F, t22182: F, t94508: F, t102486: F, t102489: F, t102495: F, t94444: F, t94460: F, t98145: F, t98147: F, t98152: F, t98157: F, t22267: F, t25997: F, t22255: F, t7264: F, t22259: F, t22276: F, t7271: F, t22281: F, t26024: F, t6876: F, t22289: F, t102498: F, t98169: F, t98174: F, t98181: F, t98186: F, t98188: F, t22115: F, t22125: F, t102515: F, t102526: F, t102527: F, t94472: F, t94474: F, t94477: F, t94479: F, t98194: F, t98203: F, t98207: F, t6864: F, t94455: F, t6846: F, t102529: F, t102549: F, t94484: F, t94498: F, t98222: F, t98227: F, t98230: F, t98236: F, t98239: F, t98244: F, t98259: F, t22061: F, t22026: F, t94550: F, t22052: F, t22056: F, t25972: F, t94520: F, t94523: F, t94526: F, t94527: F, t94537: F, t94540: F, t94546: F, t98270: F, t27932: F, t74477: F, t74419: F, t98196: F, t74423: F, t22021: F, t22068: F, t25978: F, t6880: F, t6856: F, t102569: F, t94554: F, t94565: F, t94569: F, t94571: F, t98282: F, t108530: F, t108502: F, t14230: F, t1903: F, t213: F, t22395: F, t225: F, t25930: F, t25931: F, t27868: F, t27980: F, t561: F, t7279: F, t75016: F, t94884: F, t98333: F, t98338: F, t98358: F, t98360: F, t98368: F, t98372: F, t98376: F, t98379: F, t1398: F, t543: F, t6895: F, t1904: F, t27985: F, t689: F, t108484: F, t2027: F, t2028: F, t25921: F, t26079: F, t26084: F, t30082: F, t4003: F, t545: F, t6919: F, t7295: F, t94823: F, t94914: F, t94917: F, t94919: F, t94931: F, t98382: F, t98384: F, t98387: F, t98390: F, t98399: F, t108145: F, t108172: F, t108213: F, t108233: F, t108270: F, t108310: F, t108327: F, t108349: F, t108374: F, t108399: F, t108425: F, t108443: F, t108471: F, t108500: F, t1450: F, t2014: F, t532: F) -> F {
        let (t108531, t108533, t108535, t108537, t108539, t108541, t108543) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2179::<F>(t22048, t27940, t22089, t22146, t26004, t6884, t6850, t94513, t22041, t7252, t22295, t26028);
        let t108551 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2180::<F>(t22299, t26028, t22093, t22098, t108531, t108533, t108535, t108537, t108539, t108541, t108543, t98129, t98131);
        let t108564 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2181::<F>(t2018, t22129, t807, t22262, t25986, t2661, t22182, t94508, t102486, t102489, t102495, t94444, t94460, t98145, t98147, t98152, t98157);
        let t108580 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2182::<F>(t22267, t25997, t22255, t7264, t22259, t22276, t7271, t22281, t26024, t6876, t22289, t102498, t98169, t98174, t98181, t98186, t98188);
        let t108589 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2183::<F>(t22115, t26028, t2018, t22125, t807, t102515, t102526, t102527, t94472, t94474, t94477, t94479, t98194, t98203, t98207);
        let t108596 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2184::<F>(t6864, t94455, t26024, t6846, t102529, t102549, t94484, t94498, t98222, t98227, t98230, t98236, t98239, t98244, t98259);
        let t108613 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2185::<F>(t22061, t25986, t2661, t22026, t94550, t22052, t7271, t22056, t25972, t94520, t94523, t94526, t94527, t94537, t94540, t94546, t98270);
        let (t108615, t108617, t108619, t108623, t108625, t108627) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2186::<F>(t27932, t74477, t74419, t98196, t74423, t22021, t25986, t2661, t22068, t25972, t25978, t6880);
        let t108631 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2187::<F>(t25978, t6856, t102569, t108615, t108617, t108619, t108623, t108625, t108627, t94554, t94565, t94569, t94571, t98282);
        let (t108634, t108651) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2188::<F>(t108530, t108551, t108564, t108580, t108589, t108596, t108613, t108631, t108502, t14230, t1903, t213, t22395, t225, t25930, t25931, t27868, t27980, t561, t7279, t75016, t94884, t98333, t98338, t98358, t98360, t98368, t98372, t98376, t98379);
        let t108674 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2189::<F>(t1398, t543, t6895, t1904, t27985, t689, t108484, t108634, t2027, t2028, t25921, t25931, t26079, t26084, t30082, t4003, t545, t6919, t7295, t94823, t94914, t94917, t94919, t94931, t98382, t98384, t98387, t98390, t98399);
        let t108681 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2190::<F>(t108145, t108172, t108213, t108233, t108270, t108310, t108327, t108349, t108374, t108399, t108425, t108443, t108471, t108500, t108651, t108674, t1450, t2014, t532);
    t108681
}

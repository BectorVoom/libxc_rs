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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta666(t22048: f64, t27940: f64, t22089: f64, t22146: f64, t26004: f64, t6884: f64, t6850: f64, t94513: f64, t22041: f64, t7252: f64, t22295: f64, t26028: f64, t22299: f64, t22093: f64, t22098: f64, t98129: f64, t98131: f64, t2018: f64, t22129: f64, t807: f64, t22262: f64, t25986: f64, t2661: f64, t22182: f64, t94508: f64, t102486: f64, t102489: f64, t102495: f64, t94444: f64, t94460: f64, t98145: f64, t98147: f64, t98152: f64, t98157: f64, t22267: f64, t25997: f64, t22255: f64, t7264: f64, t22259: f64, t22276: f64, t7271: f64, t22281: f64, t26024: f64, t6876: f64, t22289: f64, t102498: f64, t98169: f64, t98174: f64, t98181: f64, t98186: f64, t98188: f64, t22115: f64, t22125: f64, t102515: f64, t102526: f64, t102527: f64, t94472: f64, t94474: f64, t94477: f64, t94479: f64, t98194: f64, t98203: f64, t98207: f64, t6864: f64, t94455: f64, t6846: f64, t102529: f64, t102549: f64, t94484: f64, t94498: f64, t98222: f64, t98227: f64, t98230: f64, t98236: f64, t98239: f64, t98244: f64, t98259: f64, t22061: f64, t22026: f64, t94550: f64, t22052: f64, t22056: f64, t25972: f64, t94520: f64, t94523: f64, t94526: f64, t94527: f64, t94537: f64, t94540: f64, t94546: f64, t98270: f64, t27932: f64, t74477: f64, t74419: f64, t98196: f64, t74423: f64, t22021: f64, t22068: f64, t25978: f64, t6880: f64, t6856: f64, t102569: f64, t94554: f64, t94565: f64, t94569: f64, t94571: f64, t98282: f64, t108530: f64, t108502: f64, t14230: f64, t1903: f64, t213: f64, t22395: f64, t225: f64, t25930: f64, t25931: f64, t27868: f64, t27980: f64, t561: f64, t7279: f64, t75016: f64, t94884: f64, t98333: f64, t98338: f64, t98358: f64, t98360: f64, t98368: f64, t98372: f64, t98376: f64, t98379: f64, t1398: f64, t543: f64, t6895: f64, t1904: f64, t27985: f64, t689: f64, t108484: f64, t2027: f64, t2028: f64, t25921: f64, t26079: f64, t26084: f64, t30082: f64, t4003: f64, t545: f64, t6919: f64, t7295: f64, t94823: f64, t94914: f64, t94917: f64, t94919: f64, t94931: f64, t98382: f64, t98384: f64, t98387: f64, t98390: f64, t98399: f64, t108145: f64, t108172: f64, t108213: f64, t108233: f64, t108270: f64, t108310: f64, t108327: f64, t108349: f64, t108374: f64, t108399: f64, t108425: f64, t108443: f64, t108471: f64, t108500: f64, t1450: f64, t2014: f64, t532: f64) -> f64 {
        let (t108531, t108533, t108535, t108537, t108539, t108541, t108543) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2179(t22048, t27940, t22089, t22146, t26004, t6884, t6850, t94513, t22041, t7252, t22295, t26028);
        let t108551 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2180(t22299, t26028, t22093, t22098, t108531, t108533, t108535, t108537, t108539, t108541, t108543, t98129, t98131);
        let t108564 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2181(t2018, t22129, t807, t22262, t25986, t2661, t22182, t94508, t102486, t102489, t102495, t94444, t94460, t98145, t98147, t98152, t98157);
        let t108580 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2182(t22267, t25997, t22255, t7264, t22259, t22276, t7271, t22281, t26024, t6876, t22289, t102498, t98169, t98174, t98181, t98186, t98188);
        let t108589 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2183(t22115, t26028, t2018, t22125, t807, t102515, t102526, t102527, t94472, t94474, t94477, t94479, t98194, t98203, t98207);
        let t108596 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2184(t6864, t94455, t26024, t6846, t102529, t102549, t94484, t94498, t98222, t98227, t98230, t98236, t98239, t98244, t98259);
        let t108613 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2185(t22061, t25986, t2661, t22026, t94550, t22052, t7271, t22056, t25972, t94520, t94523, t94526, t94527, t94537, t94540, t94546, t98270);
        let (t108615, t108617, t108619, t108623, t108625, t108627) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2186(t27932, t74477, t74419, t98196, t74423, t22021, t25986, t2661, t22068, t25972, t25978, t6880);
        let t108631 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2187(t25978, t6856, t102569, t108615, t108617, t108619, t108623, t108625, t108627, t94554, t94565, t94569, t94571, t98282);
        let (t108634, t108651) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2188(t108530, t108551, t108564, t108580, t108589, t108596, t108613, t108631, t108502, t14230, t1903, t213, t22395, t225, t25930, t25931, t27868, t27980, t561, t7279, t75016, t94884, t98333, t98338, t98358, t98360, t98368, t98372, t98376, t98379);
        let t108674 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2189(t1398, t543, t6895, t1904, t27985, t689, t108484, t108634, t2027, t2028, t25921, t25931, t26079, t26084, t30082, t4003, t545, t6919, t7295, t94823, t94914, t94917, t94919, t94931, t98382, t98384, t98387, t98390, t98399);
        let t108681 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2190(t108145, t108172, t108213, t108233, t108270, t108310, t108327, t108349, t108374, t108399, t108425, t108443, t108471, t108500, t108651, t108674, t1450, t2014, t532);
    t108681
}

//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta656 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2205;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2206;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2207;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2208;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2209;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2210;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2211;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2212;
use chunk8::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2213;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta656(t1444: f64, t6862: f64, t22107: f64, t26028: f64, t22111: f64, t22271: f64, t27940: f64, t22163: f64, t6871: f64, t94429: f64, t22159: f64, t98115: f64, t22120: f64, t22076: f64, t22102: f64, t94423: f64, t22081: f64, t22085: f64, t98108: f64, t22048: f64, t22089: f64, t22146: f64, t26004: f64, t6884: f64, t6850: f64, t94513: f64, t22041: f64, t7252: f64, t22295: f64, t22299: f64, t22093: f64, t22098: f64, t98129: f64, t98131: f64, t2018: f64, t22129: f64, t807: f64, t22262: f64, t25986: f64, t2661: f64, t22182: f64, t94508: f64, t102486: f64, t102489: f64, t102495: f64, t94444: f64, t94460: f64, t98145: f64, t98147: f64, t98152: f64, t98157: f64, t22267: f64, t25997: f64, t22255: f64, t7264: f64, t22259: f64, t22276: f64, t7271: f64, t22281: f64, t26024: f64, t6876: f64, t22289: f64, t102498: f64, t98169: f64, t98174: f64, t98181: f64, t98186: f64, t98188: f64, t22115: f64, t22125: f64, t102515: f64, t102526: f64, t102527: f64, t94472: f64, t94474: f64, t94477: f64, t94479: f64, t98194: f64, t98203: f64, t98207: f64, t6864: f64, t94455: f64, t6846: f64, t102529: f64, t102549: f64, t94484: f64, t94498: f64, t98222: f64, t98227: f64, t98230: f64, t98236: f64, t98239: f64, t98244: f64, t98259: f64, t22061: f64, t22026: f64, t94550: f64, t22052: f64, t22056: f64, t25972: f64, t94520: f64, t94523: f64, t94526: f64, t94527: f64, t94537: f64, t94540: f64, t94546: f64, t98270: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t108502, t108508, t108510, t108512, t108514, t108516, t108518) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2205(t1444, t6862, t22107, t26028, t22111, t22271, t27940, t22163, t6871, t94429, t22159, t98115);
        let t108530 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2206(t22120, t26028, t22076, t22102, t94423, t22081, t22085, t108508, t108510, t108512, t108514, t108516, t108518, t98108);
        let (t108531, t108533, t108535, t108537, t108539, t108541, t108543) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2207(t22048, t27940, t22089, t22146, t26004, t6884, t6850, t94513, t22041, t7252, t22295, t26028);
        let t108551 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2208(t22299, t26028, t22093, t22098, t108531, t108533, t108535, t108537, t108539, t108541, t108543, t98129, t98131);
        let t108564 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2209(t2018, t22129, t807, t22262, t25986, t2661, t22182, t94508, t102486, t102489, t102495, t94444, t94460, t98145, t98147, t98152, t98157);
        let t108580 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2210(t22267, t25997, t22255, t7264, t22259, t22276, t7271, t22281, t26024, t6876, t22289, t102498, t98169, t98174, t98181, t98186, t98188);
        let t108589 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2211(t22115, t26028, t2018, t22125, t807, t102515, t102526, t102527, t94472, t94474, t94477, t94479, t98194, t98203, t98207);
        let t108596 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2212(t6864, t94455, t26024, t6846, t102529, t102549, t94484, t94498, t98222, t98227, t98230, t98236, t98239, t98244, t98259);
        let t108613 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2213(t22061, t25986, t2661, t22026, t94550, t22052, t7271, t22056, t25972, t94520, t94523, t94526, t94527, t94537, t94540, t94546, t98270);
    (t108502, t108530, t108551, t108564, t108580, t108589, t108596, t108613)
}

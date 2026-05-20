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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta656<F: Float>(t1444: F, t6862: F, t22107: F, t26028: F, t22111: F, t22271: F, t27940: F, t22163: F, t6871: F, t94429: F, t22159: F, t98115: F, t22120: F, t22076: F, t22102: F, t94423: F, t22081: F, t22085: F, t98108: F, t22048: F, t22089: F, t22146: F, t26004: F, t6884: F, t6850: F, t94513: F, t22041: F, t7252: F, t22295: F, t22299: F, t22093: F, t22098: F, t98129: F, t98131: F, t2018: F, t22129: F, t807: F, t22262: F, t25986: F, t2661: F, t22182: F, t94508: F, t102486: F, t102489: F, t102495: F, t94444: F, t94460: F, t98145: F, t98147: F, t98152: F, t98157: F, t22267: F, t25997: F, t22255: F, t7264: F, t22259: F, t22276: F, t7271: F, t22281: F, t26024: F, t6876: F, t22289: F, t102498: F, t98169: F, t98174: F, t98181: F, t98186: F, t98188: F, t22115: F, t22125: F, t102515: F, t102526: F, t102527: F, t94472: F, t94474: F, t94477: F, t94479: F, t98194: F, t98203: F, t98207: F, t6864: F, t94455: F, t6846: F, t102529: F, t102549: F, t94484: F, t94498: F, t98222: F, t98227: F, t98230: F, t98236: F, t98239: F, t98244: F, t98259: F, t22061: F, t22026: F, t94550: F, t22052: F, t22056: F, t25972: F, t94520: F, t94523: F, t94526: F, t94527: F, t94537: F, t94540: F, t94546: F, t98270: F) -> (F, F, F, F, F, F, F, F) {
        let (t108502, t108508, t108510, t108512, t108514, t108516, t108518) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2205::<F>(t1444, t6862, t22107, t26028, t22111, t22271, t27940, t22163, t6871, t94429, t22159, t98115);
        let t108530 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2206::<F>(t22120, t26028, t22076, t22102, t94423, t22081, t22085, t108508, t108510, t108512, t108514, t108516, t108518, t98108);
        let (t108531, t108533, t108535, t108537, t108539, t108541, t108543) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2207::<F>(t22048, t27940, t22089, t22146, t26004, t6884, t6850, t94513, t22041, t7252, t22295, t26028);
        let t108551 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2208::<F>(t22299, t26028, t22093, t22098, t108531, t108533, t108535, t108537, t108539, t108541, t108543, t98129, t98131);
        let t108564 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2209::<F>(t2018, t22129, t807, t22262, t25986, t2661, t22182, t94508, t102486, t102489, t102495, t94444, t94460, t98145, t98147, t98152, t98157);
        let t108580 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2210::<F>(t22267, t25997, t22255, t7264, t22259, t22276, t7271, t22281, t26024, t6876, t22289, t102498, t98169, t98174, t98181, t98186, t98188);
        let t108589 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2211::<F>(t22115, t26028, t2018, t22125, t807, t102515, t102526, t102527, t94472, t94474, t94477, t94479, t98194, t98203, t98207);
        let t108596 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2212::<F>(t6864, t94455, t26024, t6846, t102529, t102549, t94484, t94498, t98222, t98227, t98230, t98236, t98239, t98244, t98259);
        let t108613 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2213::<F>(t22061, t25986, t2661, t22026, t94550, t22052, t7271, t22056, t25972, t94520, t94523, t94526, t94527, t94537, t94540, t94546, t98270);
    (t108502, t108530, t108551, t108564, t108580, t108589, t108596, t108613)
}

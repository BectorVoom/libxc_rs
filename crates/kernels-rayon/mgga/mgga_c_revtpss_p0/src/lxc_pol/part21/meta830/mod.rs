//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta830 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3094;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3095;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3096;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3097;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3098;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3099;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta830(t11239: f64, t1770: f64, t13061: f64, t17225: f64, t3647: f64, t11262: f64, t1261: f64, t5303: f64, t3711: f64, t5298: f64, t127: f64, t17352: f64, t17351: f64, t17354: f64, t3588: f64, t3611: f64, t1042: f64, t1121: f64, t12273: f64, t1250: f64, t12809: f64, t12822: f64, t12945: f64, t13065: f64, t13081: f64, t17353: f64, t17412: f64, t17763: f64, t21275: f64, t3568: f64, t3620: f64, t3640: f64, t3720: f64, t44521: f64, t5277: f64, t5346: f64, t5381: f64, t5391: f64, t56713: f64, t56718: f64, t56720: f64, t56727: f64, t56728: f64, t606: f64, t5341: f64, t12904: f64, t5293: f64, t12959: f64, t17569: f64, t5269: f64, t17236: f64, t3172: f64, t17540: f64, t12956: f64, t17209: f64, t3140: f64, t5216: f64, t3599: f64, t3609: f64, t17198: f64, t12269: f64, t12800: f64, t12816: f64, t12953: f64, t17381: f64, t17710: f64, t17747: f64, t17794: f64, t17796: f64, t21203: f64, t3606: f64, t3613: f64, t44260: f64, t44664: f64, t5279: f64, t5304: f64, t53474: f64, t56246: f64, t3584: f64, t1214: f64, t17711: f64, t12773: f64, t17605: f64, t17557: f64, t17535: f64, t17728: f64, t3555: f64, t489: f64, t12772: f64, t17736: f64, t17738: f64, t12646: f64, t12855: f64, t12926: f64, t13095: f64, t17204: f64, t17214: f64, t17261: f64, t17344: f64, t17589: f64, t17732: f64, t1789: f64, t44200: f64, t44215: f64, t44500: f64, t45796: f64, t5268: f64, t5332: f64, t53459: f64, t53464: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t56730, t56731, t56734, t56740, t56742, t56756) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3094(t11239, t1770, t13061, t17225, t3647, t11262, t1261, t5303, t3711, t5298, t127, t17352);
        let t56765 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3095(t17351, t17354, t56756, t3588, t3611, t1042, t1121, t12273, t1250, t12809, t12822, t12945, t13065, t13081, t17353, t17412, t17763, t21275, t3568, t3620, t3640, t3711, t3720, t44521, t5277, t5346, t5381, t5391, t56713, t56718, t56720, t56727, t56728, t56731, t56734, t56740, t56742, t606);
        let (t56766, t56786, t56787, t56791, t56793, t56796) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3096(t3588, t5341, t12904, t5293, t12959, t17569, t11262, t1261, t5269, t17236, t3172, t17540, t3711);
        let t56818 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3097(t12956, t17209, t3140, t5216, t3599, t3609, t1261, t17198, t3172, t1042, t12269, t12800, t12816, t12953, t13081, t17381, t17569, t17710, t17747, t17794, t17796, t21203, t3606, t3613, t3711, t3720, t44260, t44664, t5279, t5304, t53474, t5381, t56246, t56766, t56786, t56787, t56791, t56793, t56796);
        let (t56825, t56830, t56835, t56838, t56853, t56861) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3098(t3584, t5341, t1214, t17711, t12773, t17605, t1261, t17557, t3172, t17535, t3711, t17728, t3555, t489);
        let t56873 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3099(t12772, t17736, t17738, t1042, t1261, t12646, t12855, t12926, t12956, t13095, t17204, t17214, t17261, t17344, t17589, t17710, t17732, t1789, t21275, t3647, t3720, t44200, t44215, t44500, t45796, t5268, t5332, t53459, t53464, t5391, t56825, t56830, t56835, t56838, t56853, t56861);
    (t56730, t56756, t56765, t56766, t56818, t56825, t56830, t56861, t56873)
}

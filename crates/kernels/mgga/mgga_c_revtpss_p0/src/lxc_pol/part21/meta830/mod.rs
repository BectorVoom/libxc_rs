//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta830 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3094;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3095;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3096;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3097;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3098;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3099;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta830<F: Float>(t11239: F, t1770: F, t13061: F, t17225: F, t3647: F, t11262: F, t1261: F, t5303: F, t3711: F, t5298: F, t127: F, t17352: F, t17351: F, t17354: F, t3588: F, t3611: F, t1042: F, t1121: F, t12273: F, t1250: F, t12809: F, t12822: F, t12945: F, t13065: F, t13081: F, t17353: F, t17412: F, t17763: F, t21275: F, t3568: F, t3620: F, t3640: F, t3720: F, t44521: F, t5277: F, t5346: F, t5381: F, t5391: F, t56713: F, t56718: F, t56720: F, t56727: F, t56728: F, t606: F, t5341: F, t12904: F, t5293: F, t12959: F, t17569: F, t5269: F, t17236: F, t3172: F, t17540: F, t12956: F, t17209: F, t3140: F, t5216: F, t3599: F, t3609: F, t17198: F, t12269: F, t12800: F, t12816: F, t12953: F, t17381: F, t17710: F, t17747: F, t17794: F, t17796: F, t21203: F, t3606: F, t3613: F, t44260: F, t44664: F, t5279: F, t5304: F, t53474: F, t56246: F, t3584: F, t1214: F, t17711: F, t12773: F, t17605: F, t17557: F, t17535: F, t17728: F, t3555: F, t489: F, t12772: F, t17736: F, t17738: F, t12646: F, t12855: F, t12926: F, t13095: F, t17204: F, t17214: F, t17261: F, t17344: F, t17589: F, t17732: F, t1789: F, t44200: F, t44215: F, t44500: F, t45796: F, t5268: F, t5332: F, t53459: F, t53464: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t56730, t56731, t56734, t56740, t56742, t56756) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3094::<F>(t11239, t1770, t13061, t17225, t3647, t11262, t1261, t5303, t3711, t5298, t127, t17352);
        let t56765 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3095::<F>(t17351, t17354, t56756, t3588, t3611, t1042, t1121, t12273, t1250, t12809, t12822, t12945, t13065, t13081, t17353, t17412, t17763, t21275, t3568, t3620, t3640, t3711, t3720, t44521, t5277, t5346, t5381, t5391, t56713, t56718, t56720, t56727, t56728, t56731, t56734, t56740, t56742, t606);
        let (t56766, t56786, t56787, t56791, t56793, t56796) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3096::<F>(t3588, t5341, t12904, t5293, t12959, t17569, t11262, t1261, t5269, t17236, t3172, t17540, t3711);
        let t56818 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3097::<F>(t12956, t17209, t3140, t5216, t3599, t3609, t1261, t17198, t3172, t1042, t12269, t12800, t12816, t12953, t13081, t17381, t17569, t17710, t17747, t17794, t17796, t21203, t3606, t3613, t3711, t3720, t44260, t44664, t5279, t5304, t53474, t5381, t56246, t56766, t56786, t56787, t56791, t56793, t56796);
        let (t56825, t56830, t56835, t56838, t56853, t56861) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3098::<F>(t3584, t5341, t1214, t17711, t12773, t17605, t1261, t17557, t3172, t17535, t3711, t17728, t3555, t489);
        let t56873 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3099::<F>(t12772, t17736, t17738, t1042, t1261, t12646, t12855, t12926, t12956, t13095, t17204, t17214, t17261, t17344, t17589, t17710, t17732, t1789, t21275, t3647, t3720, t44200, t44215, t44500, t45796, t5268, t5332, t53459, t53464, t5391, t56825, t56830, t56835, t56838, t56853, t56861);
    (t56730, t56756, t56765, t56766, t56818, t56825, t56830, t56861, t56873)
}

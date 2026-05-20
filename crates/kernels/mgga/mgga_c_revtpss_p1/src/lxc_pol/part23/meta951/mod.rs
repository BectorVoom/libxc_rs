//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta951 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3149;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3150;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3151;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3152;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3153;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3154;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta951<F: Float>(t24770: F, t73: F, t1214: F, t12809: F, t12855: F, t12910: F, t17459: F, t17605: F, t17709: F, t20800: F, t21028: F, t21119: F, t21157: F, t24704: F, t24729: F, t3625: F, t3626: F, t3629: F, t3720: F, t44738: F, t5407: F, t56727: F, t56740: F, t56742: F, t71275: F, t82293: F, t12916: F, t24752: F, t3718: F, t1261: F, t12884: F, t24232: F, t247: F, t17729: F, t17753: F, t20836: F, t20903: F, t20923: F, t20956: F, t21203: F, t21246: F, t24535: F, t24834: F, t3647: F, t44551: F, t5051: F, t5274: F, t59279: F, t6421: F, t69820: F, t70303: F, t82481: F, t82664: F, t24734: F, t1248: F, t17261: F, t17747: F, t21049: F, t21121: F, t21223: F, t24715: F, t24739: F, t3604: F, t44609: F, t5284: F, t56786: F, t56791: F, t59162: F, t6688: F, t69856: F, t69866: F, t70890: F, t1263: F, t24616: F, t24633: F, t17525: F, t21188: F, t24758: F, t3172: F, t3711: F, t24643: F, t1042: F, t1122: F, t12956: F, t17344: F, t17448: F, t1774: F, t18281: F, t21219: F, t24649: F, t24751: F, t5245: F, t5296: F, t57571: F, t5825: F, t6640: F, t17633: F, t471: F, t3153: F, t12784: F, t20272: F, t21022: F, t21228: F, t24792: F, t24794: F, t24798: F, t5340: F, t5341: F, t5402: F, t6425: F, t69885: F, t69890: F, t70995: F, t24543: F, t42871: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t82725, t82730) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3149::<F>(t24770, t73, t1214, t12809, t12855, t12910, t17459, t17605, t17709, t20800, t21028, t21119, t21157, t24704, t24729, t3625, t3626, t3629, t3720, t44738, t5407, t56727, t56740, t56742, t71275, t82293);
        let t82763 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3150::<F>(t12916, t24752, t3718, t1261, t12884, t24232, t247, t17729, t17753, t20836, t20903, t20923, t20956, t21203, t21246, t24535, t24834, t3626, t3647, t3720, t44551, t5051, t5274, t59279, t6421, t69820, t70303, t82481, t82664);
        let (t82775, t82792) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3151::<F>(t1214, t24734, t1248, t12809, t12855, t17261, t17459, t17747, t20800, t21049, t21121, t21223, t24715, t24729, t24739, t3604, t3720, t44609, t5284, t56786, t56791, t59162, t6688, t69856, t69866, t70890);
        let t82831 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3152::<F>(t1263, t24616, t24633, t17525, t21188, t24758, t3172, t3711, t1261, t24643, t1042, t1122, t12809, t12956, t17344, t17448, t1774, t18281, t21028, t21219, t24649, t24751, t3720, t5245, t5296, t57571, t5825, t6640);
        let (t82838, t82859, t82864) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3153::<F>(t17633, t471, t24770, t3153, t12784, t17605, t20272, t21022, t21228, t24792, t24794, t24798, t3625, t3626, t3720, t5340, t5341, t5402, t6425, t69885, t69890, t70995, t71275);
        let (t82881, t82886) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3154::<F>(t1248, t24616, t24543, t42871);
    (t82725, t82730, t82763, t82775, t82792, t82831, t82838, t82859, t82864, t82881, t82886)
}

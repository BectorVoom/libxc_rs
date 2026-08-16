//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta951 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3149;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3150;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3151;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3152;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3153;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3154;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta951(t24770: f64, t73: f64, t1214: f64, t12809: f64, t12855: f64, t12910: f64, t17459: f64, t17605: f64, t17709: f64, t20800: f64, t21028: f64, t21119: f64, t21157: f64, t24704: f64, t24729: f64, t3625: f64, t3626: f64, t3629: f64, t3720: f64, t44738: f64, t5407: f64, t56727: f64, t56740: f64, t56742: f64, t71275: f64, t82293: f64, t12916: f64, t24752: f64, t3718: f64, t1261: f64, t12884: f64, t24232: f64, t247: f64, t17729: f64, t17753: f64, t20836: f64, t20903: f64, t20923: f64, t20956: f64, t21203: f64, t21246: f64, t24535: f64, t24834: f64, t3647: f64, t44551: f64, t5051: f64, t5274: f64, t59279: f64, t6421: f64, t69820: f64, t70303: f64, t82481: f64, t82664: f64, t24734: f64, t1248: f64, t17261: f64, t17747: f64, t21049: f64, t21121: f64, t21223: f64, t24715: f64, t24739: f64, t3604: f64, t44609: f64, t5284: f64, t56786: f64, t56791: f64, t59162: f64, t6688: f64, t69856: f64, t69866: f64, t70890: f64, t1263: f64, t24616: f64, t24633: f64, t17525: f64, t21188: f64, t24758: f64, t3172: f64, t3711: f64, t24643: f64, t1042: f64, t1122: f64, t12956: f64, t17344: f64, t17448: f64, t1774: f64, t18281: f64, t21219: f64, t24649: f64, t24751: f64, t5245: f64, t5296: f64, t57571: f64, t5825: f64, t6640: f64, t17633: f64, t471: f64, t3153: f64, t12784: f64, t20272: f64, t21022: f64, t21228: f64, t24792: f64, t24794: f64, t24798: f64, t5340: f64, t5341: f64, t5402: f64, t6425: f64, t69885: f64, t69890: f64, t70995: f64, t24543: f64, t42871: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82725, t82730) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3149(t24770, t73, t1214, t12809, t12855, t12910, t17459, t17605, t17709, t20800, t21028, t21119, t21157, t24704, t24729, t3625, t3626, t3629, t3720, t44738, t5407, t56727, t56740, t56742, t71275, t82293);
        let t82763 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3150(t12916, t24752, t3718, t1261, t12884, t24232, t247, t17729, t17753, t20836, t20903, t20923, t20956, t21203, t21246, t24535, t24834, t3626, t3647, t3720, t44551, t5051, t5274, t59279, t6421, t69820, t70303, t82481, t82664);
        let (t82775, t82792) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3151(t1214, t24734, t1248, t12809, t12855, t17261, t17459, t17747, t20800, t21049, t21121, t21223, t24715, t24729, t24739, t3604, t3720, t44609, t5284, t56786, t56791, t59162, t6688, t69856, t69866, t70890);
        let t82831 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3152(t1263, t24616, t24633, t17525, t21188, t24758, t3172, t3711, t1261, t24643, t1042, t1122, t12809, t12956, t17344, t17448, t1774, t18281, t21028, t21219, t24649, t24751, t3720, t5245, t5296, t57571, t5825, t6640);
        let (t82838, t82859, t82864) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3153(t17633, t471, t24770, t3153, t12784, t17605, t20272, t21022, t21228, t24792, t24794, t24798, t3625, t3626, t3720, t5340, t5341, t5402, t6425, t69885, t69890, t70995, t71275);
        let (t82881, t82886) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3154(t1248, t24616, t24543, t42871);
    (t82725, t82730, t82763, t82775, t82792, t82831, t82838, t82859, t82864, t82881, t82886)
}

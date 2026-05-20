//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta844 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3157;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3158;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3159;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3160;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3161;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta844<F: Float>(t1145: F, t141: F, t56224: F, t16907: F, t698: F, t16886: F, t16889: F, t12254: F, t56179: F, t56161: F, t56157: F, t56165: F, t2439: F, t5098: F, t56248: F, t56252: F, t56256: F, t1179: F, t16831: F, t1744: F, t3477: F, t3520: F, t5155: F, t12552: F, t1749: F, t1161: F, t1169: F, t1189: F, t12418: F, t12473: F, t12504: F, t12548: F, t12556: F, t17086: F, t17089: F, t1745: F, t3447: F, t3516: F, t3524: F, t45181: F, t5143: F, t5158: F, t57808: F, t57814: F, t57816: F, t57820: F, t58005: F, t58023: F, t58053: F, t58116: F, t58129: F, t58149: F, t58177: F, t58200: F, t12486: F, t1756: F, t12485: F, t12423: F, t12476: F, t12481: F, t12487: F, t12488: F, t12501: F, t12514: F, t12553: F, t16971: F, t16974: F, t17032: F, t1757: F, t3497: F, t3521: F, t45163: F, t5181: F, t5184: F, t57831: F, t57833: F, t57835: F, t57837: F, t57840: F, t57856: F, t12428: F, t1737: F, t3495: F, t1160: F, t17020: F, t1170: F, t1187: F, t12430: F, t12431: F, t12470: F, t12491: F, t12547: F, t16982: F, t16988: F, t16992: F, t16997: F, t16998: F, t17026: F, t3453: F, t3472: F, t3496: F, t3498: F, t3515: F, t43977: F, t45174: F, t45177: F, t5146: F, t57907: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t58202, t58207, t58209, t58211, t58214, t58217, t58220, t58223) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3157::<F>(t1145, t141, t56224, t16907, t698, t16886, t16889, t12254, t56179, t56161, t56157, t56165);
        let (t58225, t58227) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3158::<F>(t2439, t5098, t56248, t56252, t56256, t58202, t58207, t58209, t58211, t58214, t58217, t58220, t58223);
        let t58250 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3159::<F>(t1179, t16831, t1744, t3477, t3520, t5155, t12552, t1749, t1161, t1169, t1189, t12418, t12473, t12504, t12548, t12556, t17086, t17089, t1745, t3447, t3516, t3524, t45181, t5143, t5158, t57808, t57814, t57816, t57820, t58005, t58023, t58053, t58116, t58129, t58149, t58177, t58200, t58227);
        let t58275 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3160::<F>(t12486, t1756, t12485, t1749, t12423, t12476, t12481, t12487, t12488, t12501, t12514, t12553, t16971, t16974, t17032, t1757, t3497, t3521, t45163, t5181, t5184, t57831, t57833, t57835, t57837, t57840, t57856);
        let t58315 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3161::<F>(t12553, t1756, t12428, t1737, t3495, t5155, t1160, t17020, t1170, t1187, t12430, t12431, t12470, t12481, t12486, t12487, t12491, t12547, t16982, t16988, t16992, t16997, t16998, t17026, t1757, t3453, t3472, t3477, t3496, t3497, t3498, t3515, t43977, t45174, t45177, t5143, t5146, t5181, t57907);
    (t58202, t58207, t58209, t58211, t58214, t58217, t58220, t58223, t58225, t58250, t58275, t58315)
}

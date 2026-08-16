//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta844 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3157;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3158;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3159;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3160;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3161;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta844(t1145: f64, t141: f64, t56224: f64, t16907: f64, t698: f64, t16886: f64, t16889: f64, t12254: f64, t56179: f64, t56161: f64, t56157: f64, t56165: f64, t2439: f64, t5098: f64, t56248: f64, t56252: f64, t56256: f64, t1179: f64, t16831: f64, t1744: f64, t3477: f64, t3520: f64, t5155: f64, t12552: f64, t1749: f64, t1161: f64, t1169: f64, t1189: f64, t12418: f64, t12473: f64, t12504: f64, t12548: f64, t12556: f64, t17086: f64, t17089: f64, t1745: f64, t3447: f64, t3516: f64, t3524: f64, t45181: f64, t5143: f64, t5158: f64, t57808: f64, t57814: f64, t57816: f64, t57820: f64, t58005: f64, t58023: f64, t58053: f64, t58116: f64, t58129: f64, t58149: f64, t58177: f64, t58200: f64, t12486: f64, t1756: f64, t12485: f64, t12423: f64, t12476: f64, t12481: f64, t12487: f64, t12488: f64, t12501: f64, t12514: f64, t12553: f64, t16971: f64, t16974: f64, t17032: f64, t1757: f64, t3497: f64, t3521: f64, t45163: f64, t5181: f64, t5184: f64, t57831: f64, t57833: f64, t57835: f64, t57837: f64, t57840: f64, t57856: f64, t12428: f64, t1737: f64, t3495: f64, t1160: f64, t17020: f64, t1170: f64, t1187: f64, t12430: f64, t12431: f64, t12470: f64, t12491: f64, t12547: f64, t16982: f64, t16988: f64, t16992: f64, t16997: f64, t16998: f64, t17026: f64, t3453: f64, t3472: f64, t3496: f64, t3498: f64, t3515: f64, t43977: f64, t45174: f64, t45177: f64, t5146: f64, t57907: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t58202, t58207, t58209, t58211, t58214, t58217, t58220, t58223) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3157(t1145, t141, t56224, t16907, t698, t16886, t16889, t12254, t56179, t56161, t56157, t56165);
        let (t58225, t58227) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3158(t2439, t5098, t56248, t56252, t56256, t58202, t58207, t58209, t58211, t58214, t58217, t58220, t58223);
        let t58250 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3159(t1179, t16831, t1744, t3477, t3520, t5155, t12552, t1749, t1161, t1169, t1189, t12418, t12473, t12504, t12548, t12556, t17086, t17089, t1745, t3447, t3516, t3524, t45181, t5143, t5158, t57808, t57814, t57816, t57820, t58005, t58023, t58053, t58116, t58129, t58149, t58177, t58200, t58227);
        let t58275 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3160(t12486, t1756, t12485, t1749, t12423, t12476, t12481, t12487, t12488, t12501, t12514, t12553, t16971, t16974, t17032, t1757, t3497, t3521, t45163, t5181, t5184, t57831, t57833, t57835, t57837, t57840, t57856);
        let t58315 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3161(t12553, t1756, t12428, t1737, t3495, t5155, t1160, t17020, t1170, t1187, t12430, t12431, t12470, t12481, t12486, t12487, t12491, t12547, t16982, t16988, t16992, t16997, t16998, t17026, t1757, t3453, t3472, t3477, t3496, t3497, t3498, t3515, t43977, t45174, t45177, t5143, t5146, t5181, t57907);
    (t58202, t58207, t58209, t58211, t58214, t58217, t58220, t58223, t58225, t58250, t58275, t58315)
}

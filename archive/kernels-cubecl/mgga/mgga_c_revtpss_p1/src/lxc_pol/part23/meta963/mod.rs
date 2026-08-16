//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta963 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3255;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3256;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3257;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3258;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3259;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3260;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta963<F: Float>(t6869: F, t73856: F, t9816: F, t9818: F, t1353: F, t22079: F, t22096: F, t22876: F, t3934: F, t3936: F, t46730: F, t48489: F, t73994: F, t73998: F, t74010: F, t74015: F, t74017: F, t74022: F, t74024: F, t74029: F, t74033: F, t74037: F, t74174: F, t74184: F, t74186: F, t74206: F, t800: F, t2661: F, t3992: F, t74026: F, t13999: F, t22843: F, t22854: F, t3989: F, t221: F, t22852: F, t3978: F, t9921: F, t13783: F, t13804: F, t1410: F, t1868: F, t1883: F, t21969: F, t22016: F, t22279: F, t4012: F, t48509: F, t48516: F, t48518: F, t48529: F, t48532: F, t48563: F, t5591: F, t5673: F, t6816: F, t73847: F, t74232: F, t74249: F, t74257: F, t828: F, t85553: F, t22956: F, t3930: F, t22886: F, t9744: F, t13790: F, t13845: F, t13847: F, t13784: F, t13789: F, t13926: F, t1872: F, t22809: F, t22848: F, t22893: F, t3944: F, t5671: F, t5689: F, t6849: F, t6862: F, t74177: F, t74264: F, t74269: F, t74271: F, t74277: F, t74279: F, t74281: F, t74288: F, t9748: F, t22837: F, t9962: F, t46671: F, t46702: F, t46723: F, t48600: F, t48604: F, t48615: F, t5627: F, t6874: F, t74290: F, t74292: F, t74299: F, t74304: F, t74319: F, t74322: F, t74341: F, t74358: F, t74362: F, t9835: F, t22860: F, t47194: F, t46760: F, t46787: F, t46800: F, t46810: F, t46817: F, t46820: F, t46824: F, t48638: F, t48645: F, t48669: F, t48686: F, t48691: F, t48692: F, t48696: F, t48700: F, t6836: F, t74364: F, t9942: F) -> (F, F, F, F, F) {
        let t85738 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3255::<F>(t6869, t73856, t9816, t9818, t1353, t22079, t22096, t22876, t3934, t3936, t46730, t48489, t73994, t73998, t74010, t74015, t74017, t74022, t74024, t74029, t74033, t74037, t74174, t74184, t74186, t74206, t800);
        let (t85741, t85752, t85764, t85778) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3256::<F>(t2661, t3992, t6869, t74026, t13999, t22843, t22854, t3989, t221, t22852, t3978, t9921);
        let t85780 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3257::<F>(t13783, t13804, t1410, t1868, t1883, t21969, t22016, t22279, t3934, t4012, t48509, t48516, t48518, t48529, t48532, t48563, t5591, t5673, t6816, t73847, t74232, t74249, t74257, t828, t85553, t85741, t85752, t85764, t85778);
        let t85830 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3258::<F>(t22956, t3930, t22886, t9744, t13790, t13845, t13847, t73856, t1353, t13783, t13784, t13789, t13926, t1410, t1872, t1883, t21969, t22809, t22848, t22893, t3934, t3936, t3944, t4012, t5591, t5671, t5689, t6816, t6849, t6862, t74177, t74264, t74269, t74271, t74277, t74279, t74281, t74288, t800, t828, t9748);
        let t85854 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3259::<F>(t22837, t9962, t13783, t3934, t3936, t46671, t46702, t46723, t48600, t48604, t48615, t5627, t5671, t6874, t74290, t74292, t74299, t74304, t74319, t74322, t74341, t74358, t74362, t85553, t9835);
        let t85871 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3260::<F>(t22860, t47194, t1410, t46760, t46787, t46800, t46810, t46817, t46820, t46824, t48638, t48645, t48669, t48686, t48691, t48692, t48696, t48700, t5591, t6836, t74364, t828, t9942);
    (t85738, t85780, t85830, t85854, t85871)
}

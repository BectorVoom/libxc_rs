//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta963 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3255;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3256;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3257;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3258;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3259;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3260;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta963(t6869: f64, t73856: f64, t9816: f64, t9818: f64, t1353: f64, t22079: f64, t22096: f64, t22876: f64, t3934: f64, t3936: f64, t46730: f64, t48489: f64, t73994: f64, t73998: f64, t74010: f64, t74015: f64, t74017: f64, t74022: f64, t74024: f64, t74029: f64, t74033: f64, t74037: f64, t74174: f64, t74184: f64, t74186: f64, t74206: f64, t800: f64, t2661: f64, t3992: f64, t74026: f64, t13999: f64, t22843: f64, t22854: f64, t3989: f64, t221: f64, t22852: f64, t3978: f64, t9921: f64, t13783: f64, t13804: f64, t1410: f64, t1868: f64, t1883: f64, t21969: f64, t22016: f64, t22279: f64, t4012: f64, t48509: f64, t48516: f64, t48518: f64, t48529: f64, t48532: f64, t48563: f64, t5591: f64, t5673: f64, t6816: f64, t73847: f64, t74232: f64, t74249: f64, t74257: f64, t828: f64, t85553: f64, t22956: f64, t3930: f64, t22886: f64, t9744: f64, t13790: f64, t13845: f64, t13847: f64, t13784: f64, t13789: f64, t13926: f64, t1872: f64, t22809: f64, t22848: f64, t22893: f64, t3944: f64, t5671: f64, t5689: f64, t6849: f64, t6862: f64, t74177: f64, t74264: f64, t74269: f64, t74271: f64, t74277: f64, t74279: f64, t74281: f64, t74288: f64, t9748: f64, t22837: f64, t9962: f64, t46671: f64, t46702: f64, t46723: f64, t48600: f64, t48604: f64, t48615: f64, t5627: f64, t6874: f64, t74290: f64, t74292: f64, t74299: f64, t74304: f64, t74319: f64, t74322: f64, t74341: f64, t74358: f64, t74362: f64, t9835: f64, t22860: f64, t47194: f64, t46760: f64, t46787: f64, t46800: f64, t46810: f64, t46817: f64, t46820: f64, t46824: f64, t48638: f64, t48645: f64, t48669: f64, t48686: f64, t48691: f64, t48692: f64, t48696: f64, t48700: f64, t6836: f64, t74364: f64, t9942: f64) -> (f64, f64, f64, f64, f64) {
        let t85738 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3255(t6869, t73856, t9816, t9818, t1353, t22079, t22096, t22876, t3934, t3936, t46730, t48489, t73994, t73998, t74010, t74015, t74017, t74022, t74024, t74029, t74033, t74037, t74174, t74184, t74186, t74206, t800);
        let (t85741, t85752, t85764, t85778) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3256(t2661, t3992, t6869, t74026, t13999, t22843, t22854, t3989, t221, t22852, t3978, t9921);
        let t85780 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3257(t13783, t13804, t1410, t1868, t1883, t21969, t22016, t22279, t3934, t4012, t48509, t48516, t48518, t48529, t48532, t48563, t5591, t5673, t6816, t73847, t74232, t74249, t74257, t828, t85553, t85741, t85752, t85764, t85778);
        let t85830 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3258(t22956, t3930, t22886, t9744, t13790, t13845, t13847, t73856, t1353, t13783, t13784, t13789, t13926, t1410, t1872, t1883, t21969, t22809, t22848, t22893, t3934, t3936, t3944, t4012, t5591, t5671, t5689, t6816, t6849, t6862, t74177, t74264, t74269, t74271, t74277, t74279, t74281, t74288, t800, t828, t9748);
        let t85854 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3259(t22837, t9962, t13783, t3934, t3936, t46671, t46702, t46723, t48600, t48604, t48615, t5627, t5671, t6874, t74290, t74292, t74299, t74304, t74319, t74322, t74341, t74358, t74362, t85553, t9835);
        let t85871 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3260(t22860, t47194, t1410, t46760, t46787, t46800, t46810, t46817, t46820, t46824, t48638, t48645, t48669, t48686, t48691, t48692, t48696, t48700, t5591, t6836, t74364, t828, t9942);
    (t85738, t85780, t85830, t85854, t85871)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1071 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3836;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3837;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3838;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3839;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3840;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3841;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1071<F: Float>(t13845: F, t13847: F, t5675: F, t73731: F, t3938: F, t9816: F, t9818: F, t13848: F, t5659: F, t22159: F, t48836: F, t22120: F, t9962: F, t13789: F, t13791: F, t13804: F, t13805: F, t13824: F, t1883: F, t22046: F, t23037: F, t3934: F, t48105: F, t48113: F, t48798: F, t5671: F, t5673: F, t73726: F, t73729: F, t46917: F, t6871: F, t22298: F, t48862: F, t48863: F, t22098: F, t22102: F, t46740: F, t13783: F, t13790: F, t1398: F, t22079: F, t22118: F, t36776: F, t4004: F, t48475: F, t49146: F, t6816: F, t6862: F, t6869: F, t9955: F, t9956: F, t22299: F, t22295: F, t22111: F, t22115: F, t13999: F, t22163: F, t22048: F, t22089: F, t13926: F, t22096: F, t3936: F, t46592: F, t48102: F, t9810: F, t22076: F, t6861: F, t9994: F, t1353: F, t5658: F, t125: F, t22252: F, t124: F, t6843: F, t3923: F, t46478: F, t3924: F, t48073: F, t48759: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t73734, t73738, t73742, t73744, t73750) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3836::<F>(t13845, t13847, t5675, t73731, t3938, t9816, t9818, t13848, t5659, t22159, t48836, t22120, t9962);
        let t73752 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3837::<F>(t13789, t13791, t13804, t13805, t13824, t1883, t22046, t23037, t3934, t48105, t48113, t48798, t5671, t5673, t73726, t73729, t73734, t73738, t73742, t73744, t73750);
        let t73791 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3838::<F>(t46917, t6871, t22298, t48862, t48863, t22098, t9962, t22102, t46740, t13783, t13789, t13790, t13791, t1398, t22079, t22118, t36776, t3934, t3938, t4004, t48475, t49146, t5671, t6816, t6862, t6869, t9955, t9956);
        let t73817 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3839::<F>(t22299, t9962, t22295, t22111, t22115, t13999, t22163, t22048, t22089, t13789, t13926, t22046, t22096, t3934, t3936, t46592, t48102, t9810);
        let (t73818, t73820, t73837, t73842, t73847, t73856, t73859) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3840::<F>(t22076, t9962, t6861, t9994, t1353, t5658, t1398, t125, t22252, t124, t6843, t3938, t9816, t9818);
        let (t73861, t73870) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3841::<F>(t3923, t46478, t13789, t13791, t13804, t13926, t1883, t22046, t22079, t36776, t3924, t3934, t3936, t3938, t48073, t48105, t48759, t49146, t5659, t5673, t6869, t73818, t73820, t73837, t73842, t73847, t73859, t9810);
    (t73752, t73791, t73817, t73820, t73837, t73842, t73847, t73856, t73861, t73870)
}

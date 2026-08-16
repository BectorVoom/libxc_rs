//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1071 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3836;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3837;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3838;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3839;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3840;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3841;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1071(t13845: f64, t13847: f64, t5675: f64, t73731: f64, t3938: f64, t9816: f64, t9818: f64, t13848: f64, t5659: f64, t22159: f64, t48836: f64, t22120: f64, t9962: f64, t13789: f64, t13791: f64, t13804: f64, t13805: f64, t13824: f64, t1883: f64, t22046: f64, t23037: f64, t3934: f64, t48105: f64, t48113: f64, t48798: f64, t5671: f64, t5673: f64, t73726: f64, t73729: f64, t46917: f64, t6871: f64, t22298: f64, t48862: f64, t48863: f64, t22098: f64, t22102: f64, t46740: f64, t13783: f64, t13790: f64, t1398: f64, t22079: f64, t22118: f64, t36776: f64, t4004: f64, t48475: f64, t49146: f64, t6816: f64, t6862: f64, t6869: f64, t9955: f64, t9956: f64, t22299: f64, t22295: f64, t22111: f64, t22115: f64, t13999: f64, t22163: f64, t22048: f64, t22089: f64, t13926: f64, t22096: f64, t3936: f64, t46592: f64, t48102: f64, t9810: f64, t22076: f64, t6861: f64, t9994: f64, t1353: f64, t5658: f64, t125: f64, t22252: f64, t124: f64, t6843: f64, t3923: f64, t46478: f64, t3924: f64, t48073: f64, t48759: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t73734, t73738, t73742, t73744, t73750) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3836(t13845, t13847, t5675, t73731, t3938, t9816, t9818, t13848, t5659, t22159, t48836, t22120, t9962);
        let t73752 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3837(t13789, t13791, t13804, t13805, t13824, t1883, t22046, t23037, t3934, t48105, t48113, t48798, t5671, t5673, t73726, t73729, t73734, t73738, t73742, t73744, t73750);
        let t73791 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3838(t46917, t6871, t22298, t48862, t48863, t22098, t9962, t22102, t46740, t13783, t13789, t13790, t13791, t1398, t22079, t22118, t36776, t3934, t3938, t4004, t48475, t49146, t5671, t6816, t6862, t6869, t9955, t9956);
        let t73817 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3839(t22299, t9962, t22295, t22111, t22115, t13999, t22163, t22048, t22089, t13789, t13926, t22046, t22096, t3934, t3936, t46592, t48102, t9810);
        let (t73818, t73820, t73837, t73842, t73847, t73856, t73859) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3840(t22076, t9962, t6861, t9994, t1353, t5658, t1398, t125, t22252, t124, t6843, t3938, t9816, t9818);
        let (t73861, t73870) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3841(t3923, t46478, t13789, t13791, t13804, t13926, t1883, t22046, t22079, t36776, t3924, t3934, t3936, t3938, t48073, t48105, t48759, t49146, t5659, t5673, t6869, t73818, t73820, t73837, t73842, t73847, t73859, t9810);
    (t73752, t73791, t73817, t73820, t73837, t73842, t73847, t73856, t73861, t73870)
}

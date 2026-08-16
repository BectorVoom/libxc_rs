//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta962 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3250;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3251;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3252;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3253;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3254;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta962(t1358: f64, t212: f64, t22964: f64, t689: f64, t13848: f64, t22893: f64, t47274: f64, t9816: f64, t22890: f64, t9962: f64, t13845: f64, t22841: f64, t73731: f64, t9818: f64, t13847: f64, t1883: f64, t73856: f64, t22895: f64, t125: f64, t22813: f64, t22857: f64, t13783: f64, t1399: f64, t22046: f64, t22079: f64, t3934: f64, t3936: f64, t3938: f64, t4003: f64, t47248: f64, t5591: f64, t5627: f64, t5659: f64, t5671: f64, t5673: f64, t6862: f64, t73726: f64, t73729: f64, t73734: f64, t73738: f64, t73742: f64, t73744: f64, t73750: f64, t22809: f64, t5658: f64, t9994: f64, t1882: f64, t6816: f64, t22953: f64, t1398: f64, t46478: f64, t1353: f64, t13784: f64, t13789: f64, t13804: f64, t22852: f64, t23037: f64, t47264: f64, t48759: f64, t5675: f64, t6869: f64, t73778: f64, t73781: f64, t73787: f64, t73789: f64, t73798: f64, t73800: f64, t73820: f64, t73847: f64, t1868: f64, t6843: f64, t22829: f64, t13926: f64, t36776: f64, t73803: f64, t73805: f64, t73811: f64, t73813: f64, t73815: f64, t73818: f64, t73842: f64, t74700: f64, t22881: f64, t13790: f64, t22274: f64, t46596: f64, t46620: f64, t46645: f64, t46652: f64, t48487: f64, t48798: f64, t6836: f64, t73859: f64, t73923: f64, t73927: f64, t73929: f64, t73951: f64, t73953: f64, t73963: f64, t73975: f64, t73985: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t85509, t85514, t85516, t85532) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3250(t1358, t212, t22964, t689, t13848, t22893, t47274, t9816, t22890, t9962, t13845, t22841, t73731, t9818);
        let (t85553, t85562) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3251(t13847, t1883, t73856, t9816, t22895, t9962, t125, t22813, t22857, t13783, t1399, t22046, t22079, t3934, t3936, t3938, t4003, t47248, t5591, t5627, t5659, t5671, t5673, t6862, t73726, t73729, t73734, t73738, t73742, t73744, t73750, t85514, t85516, t85532);
        let (t85580, t85585, t85609, t85614, t85623) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3252(t125, t22809, t5658, t9994, t1882, t6816, t22953, t1398, t46478, t1353, t13783, t13784, t13789, t13804, t1399, t22046, t22852, t23037, t3934, t3936, t3938, t47264, t48759, t5671, t5673, t5675, t6869, t73778, t73781, t73787, t73789, t73798, t73800, t73820, t73847, t85553);
        let (t85625, t85638, t85659, t85680) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3253(t1868, t5658, t6843, t9994, t6869, t73731, t9816, t9818, t22829, t9962, t1882, t13783, t13784, t13789, t13804, t13926, t1399, t1883, t23037, t36776, t3934, t3938, t5671, t5673, t5675, t73803, t73805, t73811, t73813, t73815, t73818, t73842, t74700, t85553, t85585);
        let t85709 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3254(t22881, t9962, t13783, t13790, t1398, t1883, t22274, t3934, t46596, t46620, t46645, t46652, t48487, t48798, t5671, t5673, t5675, t6836, t73859, t73923, t73927, t73929, t73951, t73953, t73963, t73975, t73985, t85609);
    (t85509, t85553, t85562, t85580, t85609, t85614, t85623, t85625, t85638, t85659, t85680, t85709)
}

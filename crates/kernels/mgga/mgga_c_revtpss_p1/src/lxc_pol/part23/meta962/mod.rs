//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta962 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3250;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3251;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3252;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3253;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3254;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta962<F: Float>(t1358: F, t212: F, t22964: F, t689: F, t13848: F, t22893: F, t47274: F, t9816: F, t22890: F, t9962: F, t13845: F, t22841: F, t73731: F, t9818: F, t13847: F, t1883: F, t73856: F, t22895: F, t125: F, t22813: F, t22857: F, t13783: F, t1399: F, t22046: F, t22079: F, t3934: F, t3936: F, t3938: F, t4003: F, t47248: F, t5591: F, t5627: F, t5659: F, t5671: F, t5673: F, t6862: F, t73726: F, t73729: F, t73734: F, t73738: F, t73742: F, t73744: F, t73750: F, t22809: F, t5658: F, t9994: F, t1882: F, t6816: F, t22953: F, t1398: F, t46478: F, t1353: F, t13784: F, t13789: F, t13804: F, t22852: F, t23037: F, t47264: F, t48759: F, t5675: F, t6869: F, t73778: F, t73781: F, t73787: F, t73789: F, t73798: F, t73800: F, t73820: F, t73847: F, t1868: F, t6843: F, t22829: F, t13926: F, t36776: F, t73803: F, t73805: F, t73811: F, t73813: F, t73815: F, t73818: F, t73842: F, t74700: F, t22881: F, t13790: F, t22274: F, t46596: F, t46620: F, t46645: F, t46652: F, t48487: F, t48798: F, t6836: F, t73859: F, t73923: F, t73927: F, t73929: F, t73951: F, t73953: F, t73963: F, t73975: F, t73985: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t85509, t85514, t85516, t85532) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3250::<F>(t1358, t212, t22964, t689, t13848, t22893, t47274, t9816, t22890, t9962, t13845, t22841, t73731, t9818);
        let (t85553, t85562) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3251::<F>(t13847, t1883, t73856, t9816, t22895, t9962, t125, t22813, t22857, t13783, t1399, t22046, t22079, t3934, t3936, t3938, t4003, t47248, t5591, t5627, t5659, t5671, t5673, t6862, t73726, t73729, t73734, t73738, t73742, t73744, t73750, t85514, t85516, t85532);
        let (t85580, t85585, t85609, t85614, t85623) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3252::<F>(t125, t22809, t5658, t9994, t1882, t6816, t22953, t1398, t46478, t1353, t13783, t13784, t13789, t13804, t1399, t22046, t22852, t23037, t3934, t3936, t3938, t47264, t48759, t5671, t5673, t5675, t6869, t73778, t73781, t73787, t73789, t73798, t73800, t73820, t73847, t85553);
        let (t85625, t85638, t85659, t85680) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3253::<F>(t1868, t5658, t6843, t9994, t6869, t73731, t9816, t9818, t22829, t9962, t1882, t13783, t13784, t13789, t13804, t13926, t1399, t1883, t23037, t36776, t3934, t3938, t5671, t5673, t5675, t73803, t73805, t73811, t73813, t73815, t73818, t73842, t74700, t85553, t85585);
        let t85709 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3254::<F>(t22881, t9962, t13783, t13790, t1398, t1883, t22274, t3934, t46596, t46620, t46645, t46652, t48487, t48798, t5671, t5673, t5675, t6836, t73859, t73923, t73927, t73929, t73951, t73953, t73963, t73975, t73985, t85609);
    (t85509, t85553, t85562, t85580, t85609, t85614, t85623, t85625, t85638, t85659, t85680, t85709)
}

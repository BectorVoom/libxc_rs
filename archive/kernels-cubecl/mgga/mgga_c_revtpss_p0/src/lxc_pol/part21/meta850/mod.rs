//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta850 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3192;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3193;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3194;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3195;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3196;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta850<F: Float>(t1261: F, t1715: F, t247: F, t44701: F, t1214: F, t17748: F, t17754: F, t12809: F, t12916: F, t17380: F, t3568: F, t5333: F, t3584: F, t5352: F, t3603: F, t1248: F, t1247: F, t1796: F, t42994: F, t17231: F, t3172: F, t1250: F, t10356: F, t12256: F, t12268: F, t12787: F, t12789: F, t12832: F, t12910: F, t12922: F, t12926: F, t12933: F, t17569: F, t17605: F, t17710: F, t17724: F, t3625: F, t3720: F, t44225: F, t44551: F, t44578: F, t44609: F, t44696: F, t44952: F, t45371: F, t471: F, t5332: F, t5346: F, t5351: F, t5381: F, t3718: F, t44546: F, t5347: F, t17785: F, t5331: F, t3650: F, t5390: F, t12915: F, t16775: F, t5384: F, t3721: F, t44799: F, t12948: F, t17377: F, t17361: F, t3708: F, t17290: F, t3678: F, t1266: F, t12866: F, t12920: F, t12931: F, t1469: F, t17254: F, t17261: F, t17736: F, t17737: F, t21035: F, t3626: F, t372: F, t44704: F, t44711: F, t44726: F, t44729: F, t44748: F, t44751: F, t44773: F, t44776: F, t5302: F) -> (F, F, F, F, F, F, F, F) {
        let (t58777, t58780, t58785, t58791, t58793) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3192::<F>(t1261, t1715, t247, t44701, t1214, t17748, t17754, t12809, t12916, t17380, t3568, t5333);
        let (t58798, t58804, t58824, t58827, t58831) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3193::<F>(t3584, t5352, t3568, t3603, t1248, t1247, t1796, t42994, t1261, t17231, t3172, t1250);
        let t58842 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3194::<F>(t10356, t12256, t12268, t12787, t12789, t12832, t12910, t12922, t12926, t12933, t17569, t17605, t17710, t17724, t3625, t3720, t44225, t44551, t44578, t44609, t44696, t44952, t45371, t471, t5332, t5346, t5351, t5381, t58777, t58780, t58785, t58791, t58793, t58798, t58804, t58824, t58827, t58831);
        let (t58851, t58853, t58863, t58868) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3195::<F>(t3718, t44546, t5347, t12916, t17785, t5331, t3650, t5390, t12915, t16775, t247, t5384);
        let (t58872, t58886) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3196::<F>(t3721, t44799, t12948, t17377, t17361, t3708, t17290, t3678, t1266, t12866, t12920, t12931, t1469, t17254, t17261, t17736, t17737, t21035, t3626, t372, t44704, t44711, t44726, t44729, t44748, t44751, t44773, t44776, t5302, t58851, t58853, t58863, t58868);
    (t58780, t58785, t58793, t58798, t58804, t58842, t58872, t58886)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta850 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3192;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3193;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3194;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3195;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3196;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta850(t1261: f64, t1715: f64, t247: f64, t44701: f64, t1214: f64, t17748: f64, t17754: f64, t12809: f64, t12916: f64, t17380: f64, t3568: f64, t5333: f64, t3584: f64, t5352: f64, t3603: f64, t1248: f64, t1247: f64, t1796: f64, t42994: f64, t17231: f64, t3172: f64, t1250: f64, t10356: f64, t12256: f64, t12268: f64, t12787: f64, t12789: f64, t12832: f64, t12910: f64, t12922: f64, t12926: f64, t12933: f64, t17569: f64, t17605: f64, t17710: f64, t17724: f64, t3625: f64, t3720: f64, t44225: f64, t44551: f64, t44578: f64, t44609: f64, t44696: f64, t44952: f64, t45371: f64, t471: f64, t5332: f64, t5346: f64, t5351: f64, t5381: f64, t3718: f64, t44546: f64, t5347: f64, t17785: f64, t5331: f64, t3650: f64, t5390: f64, t12915: f64, t16775: f64, t5384: f64, t3721: f64, t44799: f64, t12948: f64, t17377: f64, t17361: f64, t3708: f64, t17290: f64, t3678: f64, t1266: f64, t12866: f64, t12920: f64, t12931: f64, t1469: f64, t17254: f64, t17261: f64, t17736: f64, t17737: f64, t21035: f64, t3626: f64, t372: f64, t44704: f64, t44711: f64, t44726: f64, t44729: f64, t44748: f64, t44751: f64, t44773: f64, t44776: f64, t5302: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t58777, t58780, t58785, t58791, t58793) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3192(t1261, t1715, t247, t44701, t1214, t17748, t17754, t12809, t12916, t17380, t3568, t5333);
        let (t58798, t58804, t58824, t58827, t58831) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3193(t3584, t5352, t3568, t3603, t1248, t1247, t1796, t42994, t1261, t17231, t3172, t1250);
        let t58842 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3194(t10356, t12256, t12268, t12787, t12789, t12832, t12910, t12922, t12926, t12933, t17569, t17605, t17710, t17724, t3625, t3720, t44225, t44551, t44578, t44609, t44696, t44952, t45371, t471, t5332, t5346, t5351, t5381, t58777, t58780, t58785, t58791, t58793, t58798, t58804, t58824, t58827, t58831);
        let (t58851, t58853, t58863, t58868) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3195(t3718, t44546, t5347, t12916, t17785, t5331, t3650, t5390, t12915, t16775, t247, t5384);
        let (t58872, t58886) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3196(t3721, t44799, t12948, t17377, t17361, t3708, t17290, t3678, t1266, t12866, t12920, t12931, t1469, t17254, t17261, t17736, t17737, t21035, t3626, t372, t44704, t44711, t44726, t44729, t44748, t44751, t44773, t44776, t5302, t58851, t58853, t58863, t58868);
    (t58780, t58785, t58793, t58798, t58804, t58842, t58872, t58886)
}

//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta477 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2067;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2068;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2069;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta477(t1842: f64, t3911: f64, t3887: f64, t3888: f64, t12021: f64, t12033: f64, t1375: f64, t1386: f64, t16453: f64, t16458: f64, t16460: f64, t16463: f64, t16465: f64, t16468: f64, t1843: f64, t3758: f64, t3882: f64, t3889: f64, t5215: f64, t5326: f64, t5354: f64, t568: f64, t16451: f64, t3734: f64, t571: f64, t1390: f64, t5356: f64, t12127: f64, t12133: f64, t12141: f64, t12466: f64, t1297: f64, t1307: f64, t15983: f64, t15985: f64, t15987: f64, t15988: f64, t16018: f64, t16165: f64, t16166: f64, t16167: f64, t16168: f64, t16171: f64, t16172: f64, t1799: f64, t193: f64, t3918: f64, t533: f64, t9853: f64, t9859: f64, t15903: f64, t15929: f64, t15981: f64, t113: f64, t1266: f64, t1271: f64, t12724: f64, t12728: f64, t12835: f64, t12841: f64, t1393: f64, t15857: f64, t1774: f64, t1778: f64, t2312: f64, t2314: f64, t2320: f64, t3929: f64, t4026: f64, t4037: f64, t4077: f64, t510: f64, t5107: f64, t5118: f64, t513: f64, t5361: f64, t650: f64, t652: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t16471, t16475, t16485) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2067(t1842, t3911, t3887, t3888, t12021, t12033, t1375, t1386, t16453, t16458, t16460, t16463, t16465, t16468, t1843, t3758, t3882, t3889, t5215, t5326, t5354, t568);
        let (t16486, t16490, t16497, t16501) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2068(t16451, t16485, t3734, t571, t1390, t5356, t12127, t12133, t12141, t12466, t1297, t1307, t15983, t15985, t15987, t15988, t16018, t16165, t16166, t16167, t16168, t16171, t16172, t1799, t193, t3918, t533, t9853, t9859);
        let (t16503, t16505) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2069(t15903, t15929, t15981, t16501, t113, t1266, t1271, t12724, t12728, t12835, t12841, t1393, t15857, t1774, t1778, t2312, t2314, t2320, t3929, t4026, t4037, t4077, t510, t5107, t5118, t513, t5361, t650, t652);
    (t16471, t16475, t16486, t16490, t16497, t16503, t16505)
}

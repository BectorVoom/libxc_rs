//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta401 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1555;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1556;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1557;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1558;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1559;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta401(t16451: f64, t16485: f64, t3734: f64, t571: f64, t1390: f64, t5356: f64, t12127: f64, t12133: f64, t12141: f64, t12466: f64, t1297: f64, t1307: f64, t15983: f64, t15985: f64, t15987: f64, t15988: f64, t16018: f64, t16165: f64, t16166: f64, t16167: f64, t16168: f64, t16171: f64, t16172: f64, t1799: f64, t193: f64, t3918: f64, t533: f64, t9853: f64, t9859: f64, t15903: f64, t15929: f64, t15981: f64, t113: f64, t1266: f64, t1271: f64, t12724: f64, t12728: f64, t12835: f64, t12841: f64, t1393: f64, t15857: f64, t1774: f64, t1778: f64, t2312: f64, t2314: f64, t2320: f64, t3929: f64, t4026: f64, t4037: f64, t4077: f64, t510: f64, t5107: f64, t5118: f64, t513: f64, t5361: f64, t650: f64, t652: f64, t12832: f64, t3: f64, t112: f64, t5363: f64, t111: f64, t1851: f64, t2319: f64, t576: f64, t4072: f64, t671: f64, t1458: f64, t2363: f64, t12521: f64, t12524: f64, t12813: f64, t1401: f64, t3938: f64, t3941: f64, t5371: f64, t5376: f64, t577: f64, t1484: f64, t868: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16486, t16501) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1555(t16451, t16485, t3734, t571, t1390, t5356, t12127, t12133, t12141, t12466, t1297, t1307, t15983, t15985, t15987, t15988, t16018, t16165, t16166, t16167, t16168, t16171, t16172, t1799, t193, t3918, t533, t9853, t9859);
        let (t16503, t16505) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1556(t15903, t15929, t15981, t16501, t113, t1266, t1271, t12724, t12728, t12835, t12841, t1393, t15857, t1774, t1778, t2312, t2314, t2320, t3929, t4026, t4037, t4077, t510, t5107, t5118, t513, t5361, t650, t652);
        let (t16506, t16507, t16521, t16524, t16535, t16538, t16541) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1557(t12832, t16505, t3, t112, t5363, t111, t1851, t2319, t576, t4072, t671, t1458, t2363);
        let t16546 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1558(t12521, t12524, t12813, t1401, t1458, t16506, t16521, t16524, t16535, t16538, t16541, t2319, t2363, t3938, t3941, t4072, t5371, t5376, t577, t671);
        let t16596 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1559(t1484, t868);
    (t16486, t16503, t16506, t16507, t16521, t16524, t16535, t16538, t16541, t16546, t16596)
}

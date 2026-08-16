//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta453 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1816;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1817;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1818;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta453(t3919: f64, t6330: f64, t12116: f64, t12118: f64, t12123: f64, t12130: f64, t12133: f64, t12141: f64, t15976: f64, t16171: f64, t19689: f64, t19690: f64, t19691: f64, t19693: f64, t19694: f64, t19695: f64, t19696: f64, t19697: f64, t19698: f64, t5126: f64, t9853: f64, t9859: f64, t19595: f64, t20075: f64, t20092: f64, t19534: f64, t510: f64, t1458: f64, t5107: f64, t113: f64, t12725: f64, t1442: f64, t1459: f64, t1774: f64, t1778: f64, t1849: f64, t19289: f64, t19537: f64, t2314: f64, t4026: f64, t4028: f64, t4034: f64, t4073: f64, t4077: f64, t5118: f64, t513: f64, t5361: f64, t5460: f64, t574: f64, t652: f64, t7458: f64, t6287: f64, t671: f64, t4072: f64, t1266: f64, t5493: f64, t1271: f64, t1393: f64, t19450: f64, t19451: f64, t19456: f64, t19461: f64, t4037: f64, t5450: f64, t5457: f64, t5494: f64, t6295: f64, t6468: f64, t650: f64, t672: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t20096 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1816(t3919, t6330, t12116, t12118, t12123, t12130, t12133, t12141, t15976, t16171, t19689, t19690, t19691, t19693, t19694, t19695, t19696, t19697, t19698, t5126, t9853, t9859);
        let (t20098, t20100, t20109, t20118) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1817(t19595, t20075, t20092, t20096, t19534, t510, t1458, t5107, t113, t12725, t1442, t1459, t1774, t1778, t1849, t19289, t19537, t2314, t4026, t4028, t4034, t4073, t4077, t5118, t513, t5361, t5460, t574, t652, t7458);
        let (t20127, t20136, t20143, t20147) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1818(t6287, t671, t1774, t4072, t1266, t5493, t1271, t1393, t1459, t19450, t19451, t19456, t19461, t2314, t4028, t4034, t4037, t510, t5450, t5457, t5494, t6295, t6468, t650, t652, t672);
    (t20098, t20100, t20109, t20118, t20127, t20136, t20143, t20147)
}

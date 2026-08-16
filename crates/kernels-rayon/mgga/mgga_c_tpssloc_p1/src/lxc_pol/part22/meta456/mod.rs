//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta456 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1824;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1825;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1826;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1827;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta456(t20234: f64, t9287: f64, t3981: f64, t5398: f64, t20217: f64, t43: f64, t48: f64, t481: f64, t9300: f64, t3990: f64, t55: f64, t1420: f64, t1423: f64, t39: f64, t51: f64, t5416: f64, t5421: f64, t5424: f64, t56: f64, t9311: f64, sigma2: f64, t33: f64, t4007: f64, t4012: f64, t634: f64, t638: f64, t9321: f64, t9330: f64, t72: f64, t1411: f64, t1427: f64, t1434: f64, t19322: f64, t20207: f64, t20210: f64, t20219: f64, t20222: f64, t20227: f64, t5393: f64, t5400: f64, t5403: f64, t5428: f64, t5442: f64, t66: f64, t80: f64, t5: f64, t12571: f64, t1437: f64, t19299: f64, t20193: f64, t20201: f64, t20204: f64, t2240: f64, t3953: f64, t5389: f64, t5445: f64, t605: f64, t86: f64, t9239: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20235, t20238, t20241, t20246, t20264) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1824(t20234, t9287, t3981, t5398, t20217, t43, t48, t481, t9300, t3990, t55, t1420, t1423, t39, t51, t5416, t5421, t5424, t56, t9311, sigma2);
        let (t20265, t20285) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1825(t20264, t33, t20217, t20234, t4007, t4012, t5398, t634, t638, t9321, t9330, t72);
        let t20288 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1826(t1411, t1427, t1434, t19322, t20207, t20210, t20219, t20222, t20227, t20265, t20285, t5393, t5400, t5403, t5428, t5442, t66, t80);
        let t20292 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1827(t5, t12571, t1437, t19299, t20193, t20201, t20204, t20288, t2240, t3953, t5389, t5445, t605, t86, t9239);
    (t20235, t20238, t20241, t20246, t20264, t20265, t20285, t20288, t20292)
}

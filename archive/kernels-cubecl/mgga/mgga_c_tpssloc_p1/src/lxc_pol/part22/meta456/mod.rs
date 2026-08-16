//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta456 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1824;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1825;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1826;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1827;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta456<F: Float>(t20234: F, t9287: F, t3981: F, t5398: F, t20217: F, t43: F, t48: F, t481: F, t9300: F, t3990: F, t55: F, t1420: F, t1423: F, t39: F, t51: F, t5416: F, t5421: F, t5424: F, t56: F, t9311: F, sigma2: F, t33: F, t4007: F, t4012: F, t634: F, t638: F, t9321: F, t9330: F, t72: F, t1411: F, t1427: F, t1434: F, t19322: F, t20207: F, t20210: F, t20219: F, t20222: F, t20227: F, t5393: F, t5400: F, t5403: F, t5428: F, t5442: F, t66: F, t80: F, t5: F, t12571: F, t1437: F, t19299: F, t20193: F, t20201: F, t20204: F, t2240: F, t3953: F, t5389: F, t5445: F, t605: F, t86: F, t9239: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t20235, t20238, t20241, t20246, t20264) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1824::<F>(t20234, t9287, t3981, t5398, t20217, t43, t48, t481, t9300, t3990, t55, t1420, t1423, t39, t51, t5416, t5421, t5424, t56, t9311, sigma2);
        let (t20265, t20285) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1825::<F>(t20264, t33, t20217, t20234, t4007, t4012, t5398, t634, t638, t9321, t9330, t72);
        let t20288 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1826::<F>(t1411, t1427, t1434, t19322, t20207, t20210, t20219, t20222, t20227, t20265, t20285, t5393, t5400, t5403, t5428, t5442, t66, t80);
        let t20292 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1827::<F>(t5, t12571, t1437, t19299, t20193, t20201, t20204, t20288, t2240, t3953, t5389, t5445, t605, t86, t9239);
    (t20235, t20238, t20241, t20246, t20264, t20265, t20285, t20288, t20292)
}

//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta468 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1854;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1855;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1856;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1857;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta468(t1378: f64, t20661: f64, t20594: f64, t562: f64, t1834: f64, t6361: f64, t1375: f64, t1843: f64, t20029: f64, t20044: f64, t20060: f64, t20420: f64, t20602: f64, t20609: f64, t20613: f64, t5215: f64, t5321: f64, t568: f64, t6440: f64, t6461: f64, t12044: f64, t12046: f64, t12048: f64, t12053: f64, t12055: f64, t12057: f64, t12059: f64, t1297: f64, t1390: f64, t1799: f64, t193: f64, t20067: f64, t20372: f64, t20398: f64, t20416: f64, t20520: f64, t3918: f64, t533: f64, t9780: f64, t9789: f64, t5127: f64, t6347: f64, t1845: f64, t6324: f64, t5122: f64, t6330: f64, t12087: f64, t12094: f64, t12103: f64, t12105: f64, t12109: f64, t12114: f64, t12461: f64, t20523: f64, t20524: f64, t5126: f64, t9793: f64, t9797: f64, t9820: f64, t9824: f64, t12116: f64, t12118: f64, t12121: f64, t12123: f64, t12133: f64, t12141: f64, t20526: f64, t20527: f64, t20528: f64, t20529: f64, t20530: f64, t20532: f64, t9853: f64, t9859: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20662, t20670, t20672, t20675) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1854(t1378, t20661, t20594, t562, t1834, t6361, t1375, t1843, t20029, t20044, t20060, t20420, t20602, t20609, t20613, t5215, t5321, t568, t6440, t6461);
        let t20679 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1855(t12044, t12046, t12048, t12053, t12055, t12057, t12059, t1297, t1390, t1799, t193, t20067, t20372, t20398, t20416, t20520, t20675, t3918, t533, t9780, t9789);
        let (t20681, t20684, t20689, t20692) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1856(t5127, t6347, t1845, t6324, t5122, t6330, t12087, t12094, t12103, t12105, t12109, t12114, t12461, t193, t20523, t20524, t5126, t533, t9793, t9797, t9820, t9824);
        let t20696 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1857(t12116, t12118, t12121, t12123, t12133, t12141, t20526, t20527, t20528, t20529, t20530, t20532, t3918, t5122, t6347, t9853, t9859);
    (t20662, t20670, t20672, t20675, t20679, t20681, t20684, t20689, t20692, t20696)
}

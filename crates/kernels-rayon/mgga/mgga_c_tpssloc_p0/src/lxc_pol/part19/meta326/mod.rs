//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta326 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1157;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1158;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1159;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1160;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1161;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta326(t12290: f64, t3777: f64, t12247: f64, t551: f64, t236: f64, t1336: f64, t240: f64, t3791: f64, t3792: f64, t12283: f64, t12422: f64, t12339: f64, t3876: f64, t10021: f64, t1361: f64, t1369: f64, t119: f64, t12286: f64, t12293: f64, t12297: f64, t12361: f64, t1315: f64, t1343: f64, t210: f64, t3733: f64, t3783: f64, t39622: f64, t39892: f64, t40012: f64, t40019: f64, t40022: f64, t40025: f64, t40026: f64, t820: f64, t12345: f64, t22843: f64, t241: f64, t67: f64, t3872: f64, t12353: f64, t3866: f64, t12211: f64, t12375: f64, t12012: f64, t12215: f64, t12240: f64, t12305: f64, t12336: f64, t12368: f64, t1328: f64, t1363: f64, t3719: f64, t3765: f64, t3870: f64, t5246: f64, t5248: f64, t12300: f64, t3853: f64, t12238: f64, t68: f64, t1340: f64, t1339: f64, t1354: f64, t12365: f64, t3858: f64, t12379: f64, t3799: f64, t12384: f64, t3795: f64, t39937: f64, t12282: f64, t3809: f64, t12328: f64, t1333: f64, t12351: f64, t1307: f64, t3734: f64, t3790: f64, t3803: f64, t3851: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40035, t40041, t40044, t40045, t40047, t40052, t40054) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1157(t12290, t3777, t12247, t551, t236, t1336, t240, t3791, t3792, t12283, t12422, t12339, t3876);
        let t40062 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1158(t10021, t1336, t1361, t1369, t119, t12286, t12293, t12297, t12361, t1315, t1343, t210, t3733, t3783, t39622, t39892, t40012, t40019, t40022, t40025, t40026, t40035, t40044, t40047, t40052, t40054, t820);
        let t40101 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1159(t12345, t3876, t22843, t241, t67, t3872, t12353, t3866, t12339, t12211, t12375, t12012, t12215, t12240, t12305, t12336, t12368, t1328, t1363, t210, t3719, t3733, t3765, t3783, t3870, t39622, t40026, t5246, t5248, t820);
        let (t40114, t40116, t40118, t40119, t40124, t40126) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1160(t12300, t3853, t12305, t3866, t12238, t68, t1340, t10021, t1336, t1339, t1354, t12365, t3858);
        let (t40133, t40147) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1161(t12379, t3799, t12384, t3777, t3795, t3792, t39937, t12282, t3809, t12328, t1333, t12012, t12351, t12368, t1307, t1343, t1354, t1363, t3719, t3734, t3790, t3803, t3851, t3870, t40114, t40116, t40119, t40124, t40126, t5248, t820);
    (t40041, t40045, t40047, t40062, t40101, t40118, t40133, t40147)
}

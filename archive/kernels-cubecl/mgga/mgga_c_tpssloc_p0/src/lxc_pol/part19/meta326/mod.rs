//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta326 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1157;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1158;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1159;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1160;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1161;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta326<F: Float>(t12290: F, t3777: F, t12247: F, t551: F, t236: F, t1336: F, t240: F, t3791: F, t3792: F, t12283: F, t12422: F, t12339: F, t3876: F, t10021: F, t1361: F, t1369: F, t119: F, t12286: F, t12293: F, t12297: F, t12361: F, t1315: F, t1343: F, t210: F, t3733: F, t3783: F, t39622: F, t39892: F, t40012: F, t40019: F, t40022: F, t40025: F, t40026: F, t820: F, t12345: F, t22843: F, t241: F, t67: F, t3872: F, t12353: F, t3866: F, t12211: F, t12375: F, t12012: F, t12215: F, t12240: F, t12305: F, t12336: F, t12368: F, t1328: F, t1363: F, t3719: F, t3765: F, t3870: F, t5246: F, t5248: F, t12300: F, t3853: F, t12238: F, t68: F, t1340: F, t1339: F, t1354: F, t12365: F, t3858: F, t12379: F, t3799: F, t12384: F, t3795: F, t39937: F, t12282: F, t3809: F, t12328: F, t1333: F, t12351: F, t1307: F, t3734: F, t3790: F, t3803: F, t3851: F) -> (F, F, F, F, F, F, F, F) {
        let (t40035, t40041, t40044, t40045, t40047, t40052, t40054) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1157::<F>(t12290, t3777, t12247, t551, t236, t1336, t240, t3791, t3792, t12283, t12422, t12339, t3876);
        let t40062 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1158::<F>(t10021, t1336, t1361, t1369, t119, t12286, t12293, t12297, t12361, t1315, t1343, t210, t3733, t3783, t39622, t39892, t40012, t40019, t40022, t40025, t40026, t40035, t40044, t40047, t40052, t40054, t820);
        let t40101 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1159::<F>(t12345, t3876, t22843, t241, t67, t3872, t12353, t3866, t12339, t12211, t12375, t12012, t12215, t12240, t12305, t12336, t12368, t1328, t1363, t210, t3719, t3733, t3765, t3783, t3870, t39622, t40026, t5246, t5248, t820);
        let (t40114, t40116, t40118, t40119, t40124, t40126) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1160::<F>(t12300, t3853, t12305, t3866, t12238, t68, t1340, t10021, t1336, t1339, t1354, t12365, t3858);
        let (t40133, t40147) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1161::<F>(t12379, t3799, t12384, t3777, t3795, t3792, t39937, t12282, t3809, t12328, t1333, t12012, t12351, t12368, t1307, t1343, t1354, t1363, t3719, t3734, t3790, t3803, t3851, t3870, t40114, t40116, t40119, t40124, t40126, t5248, t820);
    (t40041, t40045, t40047, t40062, t40101, t40118, t40133, t40147)
}

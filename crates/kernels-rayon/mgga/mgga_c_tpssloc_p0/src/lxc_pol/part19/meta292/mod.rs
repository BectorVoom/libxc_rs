//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta292 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1065;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1066;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1067;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1068;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta292(t12407: f64, t3805: f64, t3806: f64, t12402: f64, t1352: f64, t5248: f64, t1995: f64, t67: f64, t246: f64, t3734: f64, t550: f64, t12368: f64, t3807: f64, t3777: f64, t3802: f64, t12392: f64, t12395: f64, t12397: f64, t12404: f64, t1341: f64, t1354: f64, t3778: f64, t3783: f64, t3803: f64, t3809: f64, t3853: f64, t3872: f64, t12279: f64, t12284: f64, t12286: f64, t12291: f64, t12293: f64, t12297: f64, t12301: f64, t12305: f64, t12308: f64, t12310: f64, t12313: f64, t12348: f64, t12390: f64, t1315: f64, t1363: f64, t3790: f64, t3795: f64, t5246: f64, t553: f64, t12169: f64, t12172: f64, t12179: f64, t12181: f64, t12238: f64, t12241: f64, t12244: f64, t12252: f64, t12256: f64, t12260: f64, t12267: f64, t12273: f64, t1332: f64, t1336: f64, t1381: f64, t1383: f64, t3773: f64, t3898: f64, t3902: f64, t3905: f64, t3907: f64, t3909: f64, t5334: f64, t5344: f64, t544: f64, t564: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12409, t12413, t12418, t12419, t12420, t12422, t12426) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1065(t12407, t3805, t3806, t12402, t1352, t5248, t1995, t67, t246, t3734, t550, t12368, t3807);
        let (t12429, t12432) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1066(t3777, t3802, t12392, t12395, t12397, t12404, t12409, t12413, t12422, t12426, t1341, t1354, t3778, t3783, t3803, t3809, t3853, t3872);
        let t12434 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1067(t12279, t12284, t12286, t12291, t12293, t12297, t12301, t12305, t12308, t12310, t12313, t12348, t12390, t12432, t1315, t1363, t3790, t3795, t5246);
        let (t12435, t12437) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1068(t12434, t553, t12169, t12172, t12179, t12181, t12238, t12241, t12244, t12252, t12256, t12260, t12267, t12273, t1332, t1336, t1381, t1383, t3773, t3777, t3898, t3902, t3905, t3907, t3909, t5334, t5344, t544, t564);
    (t12409, t12413, t12418, t12419, t12420, t12422, t12426, t12429, t12434, t12435, t12437)
}

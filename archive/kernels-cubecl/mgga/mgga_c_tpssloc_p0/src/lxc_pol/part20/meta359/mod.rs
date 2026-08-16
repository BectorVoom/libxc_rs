//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta359 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1679;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1680;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1681;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1682;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1683;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta359<F: Float>(t12419: F, t12420: F, t3806: F, t12368: F, t3805: F, t3807: F, t3777: F, t3802: F, t12392: F, t12395: F, t12397: F, t12404: F, t12409: F, t12413: F, t1341: F, t1354: F, t3778: F, t3783: F, t3803: F, t3809: F, t3853: F, t3872: F, t12279: F, t12284: F, t12286: F, t12291: F, t12293: F, t12297: F, t12301: F, t12305: F, t12308: F, t12310: F, t12313: F, t12348: F, t12390: F, t1315: F, t1363: F, t3790: F, t3795: F, t5246: F, t553: F, t12169: F, t12172: F, t12179: F, t12181: F, t12238: F, t12241: F, t12244: F, t12252: F, t12256: F, t12260: F, t12267: F, t12273: F, t1332: F, t1336: F, t1381: F, t1383: F, t3773: F, t3898: F, t3902: F, t3905: F, t3907: F, t3909: F, t5334: F, t5344: F, t544: F, t564: F, t1378: F, t12237: F, t562: F, t539: F, t225: F, t3755: F, t12016: F, t12023: F, t12027: F, t12030: F, t12033: F, t12036: F, t1375: F, t1386: F, t3758: F, t3882: F, t3889: F, t3912: F, t568: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12422, t12426, t12429) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1679::<F>(t12419, t12420, t3806, t12368, t3805, t3807, t3777, t3802);
        let t12432 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1680::<F>(t12392, t12395, t12397, t12404, t12409, t12413, t12422, t12426, t12429, t1341, t1354, t3778, t3783, t3803, t3809, t3853, t3872);
        let t12434 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1681::<F>(t12279, t12284, t12286, t12291, t12293, t12297, t12301, t12305, t12308, t12310, t12313, t12348, t12390, t12432, t1315, t1363, t3790, t3795, t5246);
        let (t12435, t12437) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1682::<F>(t12434, t553, t12169, t12172, t12179, t12181, t12238, t12241, t12244, t12252, t12256, t12260, t12267, t12273, t1332, t1336, t1381, t1383, t3773, t3777, t3898, t3902, t3905, t3907, t3909, t5334, t5344, t544, t564);
        let (t12438, t12440, t12442, t12444, t12451) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1683::<F>(t12437, t1378, t12237, t562, t12434, t539, t225, t3755, t12016, t12023, t12027, t12030, t12033, t12036, t1375, t1386, t3758, t3882, t3889, t3912, t568);
    (t12422, t12426, t12429, t12434, t12435, t12437, t12438, t12440, t12442, t12444, t12451)
}

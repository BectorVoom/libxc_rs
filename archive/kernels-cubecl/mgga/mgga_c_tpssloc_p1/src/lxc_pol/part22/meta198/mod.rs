//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta198 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1159;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1160;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1161;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1162;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1163;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1164;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta198<F: Float>(t340: F, t5836: F, t343: F, t974: F, t1597: F, t2969: F, t2986: F, t4507: F, t4529: F, t5818: F, t5821: F, t5825: F, t5829: F, t973: F, t381: F, t1603: F, t1625: F, t1044: F, t248: F, t5685: F, t3062: F, t5677: F, t5691: F, t5693: F, t5697: F, t5729: F, t5732: F, t5798: F, t5800: F, t5802: F, t5806: F, t5810: F, t5814: F, t360: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5838, t5839, t5842) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1159::<F>(t340, t5836, t343, t974, t1597);
        let (t5844, t5845, t5848) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1160::<F>(t340, t5842, t343, t974, t2969, t2986, t4507, t4529, t5818, t5821, t5825, t5829, t5839, t973);
        let (t5849, t5851, t5857) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1161::<F>(t381, t5848, t1603, t1625, t1044, t248, t5685);
        let t5861 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1162::<F>(t248, t3062, t5677);
        let t5866 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1163::<F>(t5691, t5693, t5697, t5729, t5732, t5798, t5800, t5802, t5806, t5810, t5814);
        let t5867 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1164::<F>(t360, t5866);
    (t5838, t5839, t5842, t5844, t5845, t5848, t5849, t5851, t5857, t5861, t5866, t5867)
}

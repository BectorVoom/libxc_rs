//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta60 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk365;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk366;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk367;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk368;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk369;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk370;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta60<F: Float>(t1543: F, t291: F, t1541: F, t880: F, t894: F, t901: F, t1539: F, t908: F, t136: F, t899: F, t907: F, t913: F, t893: F, t917: F, t926: F, t929: F, t932: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1545, t1547) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk365::<F>(t1543, t291, t1541, t880);
        let (t1548, t1551, t1553, t1554, t1556) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk366::<F>(t1547, t894, t901, t1539, t908, t136, t1541, t899, t907);
        let t1557 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk367::<F>(t1556, t913);
        let (t1559, t1561) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk368::<F>(t1557, t893, t1541, t917);
        let t1568 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk369::<F>(t1541, t1548, t1551, t1554, t926, t929);
        let t1569 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk370::<F>(t1568, t932);
    (t1545, t1547, t1548, t1551, t1553, t1554, t1556, t1557, t1559, t1561, t1568, t1569)
}

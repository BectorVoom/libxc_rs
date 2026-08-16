//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta83 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk586;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk587;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk588;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk589;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk590;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta83<F: Float>(t1742: F, t479: F, t471: F, t1230: F, t1653: F, t248: F, t1174: F, t1195: F, t1213: F, t1224: F, t1227: F, t1706: F, t1726: F, t1731: F, t1737: F, t467: F, t488: F, t466: F, t1734: F, t491: F, t1246: F, t493: F, t1244: F, t1729: F, t470: F, t494: F, t1241: F, t265: F, t504: F, t1238: F, t1721: F, t498: F, t1256: F, t1534: F, t1659: F, t1673: F, t1699: F, t1701: F, t1705: F, t193: F, t336: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1743, t1744, t1748, t1751) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk586::<F>(t1742, t479, t471, t1230, t1653, t248, t1174, t1195, t1213, t1224, t1227, t1706, t1726, t1731, t1737, t467, t488);
        let (t1752, t1755) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk587::<F>(t1751, t466, t1734, t491);
        let (t1756, t1758, t1760) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk588::<F>(t1246, t1755, t1751, t493, t1244, t1729, t470, t494);
        let t1761 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk589::<F>(t1241, t1760);
        let (t1763, t1768) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk590::<F>(t265, t504, t1238, t1721, t1752, t1761, t498, t1256, t1534, t1659, t1673, t1699, t1701, t1705, t193, t336);
    (t1743, t1744, t1748, t1751, t1752, t1755, t1756, t1758, t1760, t1761, t1763, t1768)
}

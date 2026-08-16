//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta83 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk586;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk587;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk588;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk589;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk590;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta83(t1742: f64, t479: f64, t471: f64, t1230: f64, t1653: f64, t248: f64, t1174: f64, t1195: f64, t1213: f64, t1224: f64, t1227: f64, t1706: f64, t1726: f64, t1731: f64, t1737: f64, t467: f64, t488: f64, t466: f64, t1734: f64, t491: f64, t1246: f64, t493: f64, t1244: f64, t1729: f64, t470: f64, t494: f64, t1241: f64, t265: f64, t504: f64, t1238: f64, t1721: f64, t498: f64, t1256: f64, t1534: f64, t1659: f64, t1673: f64, t1699: f64, t1701: f64, t1705: f64, t193: f64, t336: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1743, t1744, t1748, t1751) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk586(t1742, t479, t471, t1230, t1653, t248, t1174, t1195, t1213, t1224, t1227, t1706, t1726, t1731, t1737, t467, t488);
        let (t1752, t1755) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk587(t1751, t466, t1734, t491);
        let (t1756, t1758, t1760) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk588(t1246, t1755, t1751, t493, t1244, t1729, t470, t494);
        let t1761 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk589(t1241, t1760);
        let (t1763, t1768) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk590(t265, t504, t1238, t1721, t1752, t1761, t498, t1256, t1534, t1659, t1673, t1699, t1701, t1705, t193, t336);
    (t1743, t1744, t1748, t1751, t1752, t1755, t1756, t1758, t1760, t1761, t1763, t1768)
}

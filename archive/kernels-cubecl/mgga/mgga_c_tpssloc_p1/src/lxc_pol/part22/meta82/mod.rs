//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta82 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk566;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk567;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk568;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk569;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk570;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk571;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk572;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk573;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk574;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta82<F: Float>(t1730: F, t484: F, t1659: F, t1673: F, t1699: F, t1701: F, t1705: F, t475: F, t1214: F, t248: F, t46: F, t480: F, t47: F, t479: F, t471: F, t1230: F, t1653: F, t1174: F, t1195: F, t1213: F, t1224: F, t1227: F, t1706: F, t1726: F, t467: F, t488: F, t466: F, t491: F, t1246: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1731, t1734) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk566::<F>(t1730, t484, t1659, t1673, t1699, t1701, t1705);
        let t1735 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk567::<F>(t1734, t475);
        let t1737 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk568::<F>(t1214, t1735, t248);
        let t1742 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk569::<F>(t46, t480, t47);
        let t1743 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk570::<F>(t1742, t479);
        let (t1744, t1748) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk571::<F>(t1743, t471, t1230, t1653, t248);
        let t1751 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk572::<F>(t1174, t1195, t1213, t1224, t1227, t1706, t1726, t1731, t1737, t1744, t1748, t467, t488);
        let (t1752, t1755) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk573::<F>(t1751, t466, t1734, t491);
        let t1756 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk574::<F>(t1246, t1755);
    (t1731, t1734, t1735, t1737, t1742, t1743, t1744, t1748, t1751, t1752, t1755, t1756)
}

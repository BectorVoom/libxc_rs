//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta61 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk371;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk372;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk373;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk374;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk375;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk376;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta61(t1541: f64, t936: f64, t324: f64, t1548: f64, t1551: f64, t1554: f64, t945: f64, t948: f64, t951: f64, t1545: f64, t1559: f64, t1561: f64, t1569: f64, t300: f64, t311: f64, t924: f64, t943: f64, t942: f64, t959: f64, t1409: f64, t978: f64, t977: f64, t906: f64, t340: f64, t343: f64, t974: f64, t971: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1573 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk371(t1541, t936);
        let (t1574, t1580) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk372(t1573, t324, t1541, t1548, t1551, t1554, t945, t948);
        let t1581 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk373(t1580, t951);
        let (t1585, t1587, t1589) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk374(t1545, t1559, t1561, t1569, t1574, t1581, t300, t311, t924, t943, t1580, t942, t951);
        let (t1591, t1592, t1593, t1597) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk375(t1589, t959, t1409, t978, t977, t1554, t906);
        let (t1599, t1603) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk376(t1597, t340, t343, t974, t1593, t971, t973);
    (t1573, t1580, t1581, t1585, t1587, t1589, t1591, t1592, t1597, t1599, t1603)
}

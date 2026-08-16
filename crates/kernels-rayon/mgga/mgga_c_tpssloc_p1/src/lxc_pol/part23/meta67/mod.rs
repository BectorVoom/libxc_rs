//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta67 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk403;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk404;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk405;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk406;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk407;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk408;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta67(t1730: f64, t484: f64, t1659: f64, t1673: f64, t1699: f64, t1701: f64, t1705: f64, t475: f64, t1214: f64, t248: f64, t46: f64, t480: f64, t47: f64, t479: f64, t471: f64, t1230: f64, t1653: f64, t1174: f64, t1195: f64, t1213: f64, t1224: f64, t1227: f64, t1706: f64, t1726: f64, t467: f64, t488: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1731, t1734) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk403(t1730, t484, t1659, t1673, t1699, t1701, t1705);
        let t1735 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk404(t1734, t475);
        let t1737 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk405(t1214, t1735, t248);
        let (t1740, t1742) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk406(t46, t480, t47);
        let (t1743, t1744, t1748) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk407(t1742, t479, t471, t1230, t1653, t248);
        let t1751 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk408(t1174, t1195, t1213, t1224, t1227, t1706, t1726, t1731, t1737, t1744, t1748, t467, t488);
    (t1731, t1734, t1735, t1737, t1740, t1742, t1743, t1744, t1748, t1751)
}

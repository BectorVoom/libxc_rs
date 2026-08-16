//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta86 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk548;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk549;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk550;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk551;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk552;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk553;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk554;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta86(t1730: f64, t484: f64, t1659: f64, t1673: f64, t1699: f64, t1701: f64, t1705: f64, t475: f64, t1214: f64, t248: f64, t46: f64, t480: f64, t47: f64, t479: f64, t471: f64, t1230: f64, t1653: f64, t1174: f64, t1195: f64, t1213: f64, t1224: f64, t1227: f64, t1706: f64, t1726: f64, t467: f64, t488: f64, t466: f64, t491: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1731, t1734) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk548(t1730, t484, t1659, t1673, t1699, t1701, t1705);
        let t1735 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk549(t1734, t475);
        let t1737 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk550(t1214, t1735, t248);
        let t1742 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk551(t46, t480, t47);
        let (t1743, t1744, t1748) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk552(t1742, t479, t471, t1230, t1653, t248);
        let t1751 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk553(t1174, t1195, t1213, t1224, t1227, t1706, t1726, t1731, t1737, t1744, t1748, t467, t488);
        let (t1752, t1755) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk554(t1751, t466, t1734, t491);
    (t1731, t1734, t1735, t1737, t1742, t1743, t1744, t1748, t1751, t1752, t1755)
}

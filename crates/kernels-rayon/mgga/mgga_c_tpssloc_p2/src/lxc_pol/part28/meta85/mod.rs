//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta85 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk533;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk534;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk535;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta85(t1214: f64, t1735: f64, t248: f64, t46: f64, t480: f64, t47: f64, t479: f64, t471: f64, t1230: f64, t1653: f64, t1174: f64, t1195: f64, t1213: f64, t1224: f64, t1227: f64, t1706: f64, t1726: f64, t1731: f64, t467: f64, t488: f64, t466: f64, t1734: f64, t491: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1737, t1742) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk533(t1214, t1735, t248, t46, t480, t47);
        let (t1743, t1744, t1748, t1751) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk534(t1742, t479, t471, t1230, t1653, t248, t1174, t1195, t1213, t1224, t1227, t1706, t1726, t1731, t1737, t467, t488);
        let (t1752, t1755) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk535(t1751, t466, t1734, t491);
    (t1737, t1742, t1743, t1744, t1748, t1751, t1752, t1755)
}

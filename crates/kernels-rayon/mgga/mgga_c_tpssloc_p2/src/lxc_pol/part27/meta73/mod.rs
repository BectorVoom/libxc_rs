//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta73 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk498;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk499;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk500;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk501;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta73(t1409: f64, t55: f64, t1414: f64, t1420: f64, t39: f64, t51: f64, t56: f64, t627: f64, t33: f64, t634: f64, t638: f64, t72: f64, t1411: f64, t66: f64, t80: f64, t5: f64, t1406: f64, t605: f64, t86: f64, t112: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1426, t1427, t1430, t1431, t1433) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk498(t1409, t55, t1414, t1420, t39, t51, t56, t627, t33, t634, t638);
        let (t1434, t1437) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk499(t1433, t72, t1411, t1427, t66, t80);
        let t1441 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk500(t5, t1406, t1437, t605, t86);
        let t1442 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk501(t112, t1441);
    (t1426, t1427, t1430, t1431, t1433, t1434, t1437, t1441, t1442)
}

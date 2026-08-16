//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta267 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1111;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1112;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1113;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta267(t1530: f64, t25: f64, t1597: f64, t343: f64, t1484: f64, t28: f64, t1458: f64, t88: f64, t1778: f64, t191: f64, t192: f64, t1390: f64, t1799: f64, t6890: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7545, t7577, t7649, t7656, t7676) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1111(t1530, t25, t1597, t343, t1484, t28, t1458, t88);
        let (t7684, t7685) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1112(t1778, t191, t192);
        let (t7687, t7691) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1113(t1390, t1799, t6890);
    (t7545, t7577, t7649, t7656, t7676, t7684, t7685, t7687, t7691)
}

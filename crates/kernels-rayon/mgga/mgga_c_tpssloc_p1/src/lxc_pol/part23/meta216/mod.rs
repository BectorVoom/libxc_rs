//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta216 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk861;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk862;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta216(t13797: f64, t344: f64, t135: f64, t340: f64, t10189: f64, t1597: f64, t10224: f64, t1592: f64, t973: f64, t1599: f64, t698: f64, t10508: f64, t1616: f64, t248: f64, t1020: f64, t122: f64, t247: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13798, t13822, t13847, t13896, t13909, t13965) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk861(t13797, t344, t135, t340, t10189, t1597, t10224, t1592, t973, t1599, t698, t10508, t1616, t248);
        let (t13966, t13969) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk862(t1020, t13965, t122, t247);
    (t13798, t13822, t13847, t13896, t13909, t13965, t13966, t13969)
}

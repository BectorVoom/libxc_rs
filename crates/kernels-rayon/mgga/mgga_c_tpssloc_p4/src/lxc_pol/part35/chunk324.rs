//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 324/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk324(t1410: f64, t65: f64, t1409: f64, t43: f64, t46: f64, t48: f64, rho1: f64) -> (f64, f64, f64, f64) {
    let t1411 = t1410 * t65;
    let t1414 = t43 * t1409;
    let t1417 = t46 * rho1;
    let t1419 = 1.0_f64 / t48 / t1417;
    (t1411, t1414, t1417, t1419)
}

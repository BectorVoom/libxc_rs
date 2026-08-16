//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2047/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2047(t1351: f64, t1799: f64, t3807: f64, t16224: f64, t12289: f64, t242: f64, t1336: f64) -> (f64, f64, f64, f64, f64) {
    let t16225 = t1799 * t1351;
    let t16226 = t16225 * t3807;
    let t16227 = t16224 * t16226;
    let t16232 = t12289 * t242;
    let t16233 = t1336 * t16232;
    (t16225, t16226, t16227, t16232, t16233)
}

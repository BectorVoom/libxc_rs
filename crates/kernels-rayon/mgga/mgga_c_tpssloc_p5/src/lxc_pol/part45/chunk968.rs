//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 968/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk968(t2717: f64, t7106: f64, t1888: f64, t23270: f64, t865: f64, t31334: f64, t6579: f64, t22986: f64, t2553: f64, t31337: f64, t23185: f64, t31333: f64, t82074: f64) -> (f64, f64, f64, f64) {
    let t114601 = t2717 * t7106;
    let t114604 = t1888 * t23270 * t114601 * t865;
    let t114606 = t6579 * t31334;
    let t114610 = t22986 * t23270 * t31337 * t2553;
    let t114613 = t23185 * t82074 * t31333;
    (t114604, t114606, t114610, t114613)
}

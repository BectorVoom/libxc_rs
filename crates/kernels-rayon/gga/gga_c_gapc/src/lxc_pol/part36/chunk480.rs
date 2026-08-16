//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 480/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk480(t2723: f64, t298: f64, t181: f64, t604: f64, t892: f64, t1636: f64, t291: f64, t906: f64, t2404: f64, t966: f64, t330: f64, t197: f64) -> (f64, f64, f64, f64) {
    let t2724 = t298 * t2723;
    let t2725 = t181 * t2724;
    let t2728 = t604 * t892;
    let t2732 = t1636 * t291 * t906;
    let t2735 = t966 * t2404;
    let t2736 = t330 * t2735;
    let t2737 = t197 * t2736;
    (t2725, t2728, t2732, t2737)
}

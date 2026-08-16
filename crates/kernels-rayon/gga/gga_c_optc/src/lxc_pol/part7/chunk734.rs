//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 734/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk734(t2120: f64, t2136: f64, t146: f64, t2111: f64, t622: f64, t2116: f64, t3519: f64, t6803: f64, t3491: f64, t6909: f64, t2164: f64, t2171: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7034 = t2120 * t2136;
    let t7037 = t146 * t2111 * t622;
    let t7038 = t7037 * t2116;
    let t7040 = t3519 * t6803;
    let t7043 = t3491 * t6909;
    let t7046 = t2164 * t2171;
    (t7034, t7037, t7038, t7040, t7043, t7046)
}

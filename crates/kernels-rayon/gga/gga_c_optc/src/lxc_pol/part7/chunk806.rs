//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 806/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk806(t7592: f64, t7523: f64, t2284: f64, t7359: f64, t25: f64) -> (f64, f64, f64, f64) {
    let t7593 = 0.36793333333333333333e0_f64 * t7592;
    let t7594 = 0.93932222222222222223e0_f64 * t7523;
    let t7595 = t2284 * t7359;
    let t7596 = t25 * t7595;
    (t7593, t7594, t7595, t7596)
}

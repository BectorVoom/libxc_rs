//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 426/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk426(t2257: f64, t8: f64, t864: f64, t2280: f64, t827: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2422 = 0.12361111111111111111e-1_f64 * t2257;
    let t2434 = t8 * t864;
    let t2444 = 0.23744444444444444444e-1_f64 * t2257;
    let t2454 = 0.40256666666666666667e0_f64 * t2257;
    let t2461 = 0.137975e0_f64 * t2280;
    let t2471 = t827 * t827;
    (t2422, t2434, t2444, t2454, t2461, t2471)
}

//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 469/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk469(t2843: f64, t2916: f64, t406: f64, t2865: f64, t1084: f64) -> (f64, f64, f64, f64, f64) {
    let t3024 = 0.12361111111111111111e-1_f64 * t2843;
    let t3035 = t406 * t2916;
    let t3041 = 0.40256666666666666667e0_f64 * t2843;
    let t3048 = 0.137975e0_f64 * t2865;
    let t3057 = t1084 * t1084;
    (t3024, t3035, t3041, t3048, t3057)
}

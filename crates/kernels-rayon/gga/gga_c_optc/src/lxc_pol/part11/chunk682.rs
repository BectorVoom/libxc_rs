//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 682/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk682(t6461: f64, t6523: f64, t60: f64, t40: f64, t47: f64, t768: f64, t1026: f64, t52: f64, t542: f64, t8: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6524 = t6461 + t6523;
    let t6525 = t60 * t6524;
    let t6526 = t40 * t6525;
    let t6533 = 1.0_f64 / t47 / t768;
    let t6547 = 1.0_f64 / t52 / t1026;
    let t6567 = 1.0_f64 / t8 / t542;
    (t6524, t6525, t6526, t6533, t6547, t6567)
}

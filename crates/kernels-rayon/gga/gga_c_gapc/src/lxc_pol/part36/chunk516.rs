//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 516/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk516(t1673: f64, t3021: f64, t1043: f64, t677: f64, t191: f64, t424: f64, t1046: f64, t1938: f64, t599: f64, t596: f64, t1936: f64, t611: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3022 = t3021 * t1673;
    let t3023 = t1043 * t3022;
    let t3025 = t1043 * t677;
    let t3028 = t424 * t191;
    let t3029 = t3028 * t1046;
    let t3031 = t1938 * t599;
    let t3032 = t596 * t3031;
    let t3034 = t611 * t1936;
    (t3022, t3023, t3025, t3028, t3029, t3031, t3032, t3034)
}

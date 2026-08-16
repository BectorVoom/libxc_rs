//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 583/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk583(t265: f64, t4780: f64, t241: f64, t1343: f64, t3657: f64, t1333: f64) -> (f64, f64, f64, f64) {
    let t4781 = t4780 * t265;
    let t4783 = 0.19751789702565206229e-1_f64 * t241 * t4781;
    let t4785 = 2.0_f64 * t3657 * t1343;
    let t4786 = t1333 * t1333;
    (t4781, t4783, t4785, t4786)
}

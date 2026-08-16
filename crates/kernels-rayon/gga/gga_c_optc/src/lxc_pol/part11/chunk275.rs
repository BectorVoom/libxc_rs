//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 275/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk275(t1023: f64, t398: f64, t393: f64, t1049: f64, t401: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1060 = 0.17123333333333333333e-1_f64 * t1023;
    let t1065 = t398 * t398;
    let t1066 = 1.0_f64 / t1065;
    let t1067 = t393 * t1066;
    let t1069 = 0.516475e0_f64 * t1023;
    let t1072 = 0.104195e0_f64 * t1049;
    let t1075 = 1.0_f64 / t401;
    (t1060, t1065, t1066, t1067, t1069, t1072, t1075)
}

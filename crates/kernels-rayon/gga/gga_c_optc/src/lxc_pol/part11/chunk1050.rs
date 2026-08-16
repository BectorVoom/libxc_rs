//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1050/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1050(t2849: f64, t3107: f64, t449: f64, t508: f64, t24502: f64, t465: f64, t3145: f64, t8428: f64, t3102: f64, t26255: f64, t8425: f64, t310: f64, t3648: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26888 = t3107 * t2849;
    let t26910 = t508 * t449;
    let t26940 = t465 * t24502;
    let t26989 = t3145 * t8428;
    let t27031 = t3102 * t24502;
    let t27037 = t8425 * t26255;
    let t27059 = t310 * t3648 * t449;
    (t26888, t26910, t26940, t26989, t27031, t27037, t27059)
}

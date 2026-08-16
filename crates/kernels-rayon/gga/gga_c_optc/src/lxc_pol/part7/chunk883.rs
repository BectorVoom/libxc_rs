//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 883/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk883(t6548: f64, t8482: f64, t322: f64, t449: f64, t9: f64, t3105: f64, t3109: f64, t3103: f64, t2855: f64, t553: f64, t1900: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8483 = t8482 * t6548;
    let t8484 = t322 * t8483;
    let t8487 = t9 * t449;
    let t8488 = t8487 * t3105;
    let t8489 = t8488 * t3109;
    let t8490 = t3103 * t8489;
    let t8492 = t2855 * t553;
    let t8493 = t8492 * t1900;
    (t8483, t8484, t8487, t8488, t8490, t8493)
}

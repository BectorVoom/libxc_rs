//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 752/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk752(t2002: f64, t6: f64, t4: f64, t133: f64, t5: f64, t21: f64, t1339: f64, t1782: f64) -> (f64, f64, f64, f64) {
    let t10194 = t6 * t2002;
    let t10195 = t4 * t10194;
    let t10344 = t5 * t133;
    let t10345 = t21 * t10344;
    let t10348 = t1782 * t1339;
    (t10194, t10195, t10345, t10348)
}

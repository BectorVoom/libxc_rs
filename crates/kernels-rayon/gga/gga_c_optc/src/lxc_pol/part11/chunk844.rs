//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 844/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk844(t16257: f64, t16318: f64, t16319: f64, t6359: f64, t6437: f64, t6449: f64, t6457: f64, t6638: f64, t6644: f64, t6696: f64, t6709: f64, t6741: f64, t6747: f64) -> f64 {
    let t16334 = -t16257 - t6638 - t6644 + t6696 - t6709 + t6359 + t16318 - t16319 - t6437 + t6449 + t6457 + t6741 - t6747;
    t16334
}

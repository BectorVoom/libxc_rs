//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 620/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk620(t2922: f64, t4068: f64, t5108: f64, t5112: f64, t5115: f64, t1471: f64) -> (f64, f64) {
    let t5117 = t2922 + 0.11415555555555555555e-1_f64 * t4068 - 0.11415555555555555555e-1_f64 * t5108 + 0.34246666666666666666e-1_f64 * t5112 - 0.17123333333333333333e-1_f64 * t5115;
    let t5122 = t1471 * t1471;
    (t5117, t5122)
}

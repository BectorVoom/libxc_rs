//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 872/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk872(t2296: f64, t2301: f64, t2302: f64, t2315: f64, t350: f64, t8333: f64, t8335: f64, t8338: f64, t8345: f64, t8346: f64, t8349: f64, t8376: f64, t974: f64, t979: f64) -> f64 {
    let t8378 = -3.0_f64 * t2296 * t2315 + 6.0_f64 * t2301 * t8349 + 6.0_f64 * t8338 * t2302 + t8333 * t350 - 3.0_f64 * t8335 * t979 - 6.0_f64 * t8345 * t8346 - t974 * t8376;
    t8378
}

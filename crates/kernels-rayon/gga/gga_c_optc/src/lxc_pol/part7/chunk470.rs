//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 470/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk470(t2294: f64, t2296: f64, t2301: f64, t2302: f64, t2315: f64, t350: f64, t974: f64, t979: f64, t275: f64, t176: f64, sigma0: f64) -> (f64, f64) {
    let t2317 = t2294 * t350 - 2.0_f64 * t2296 * t979 + 2.0_f64 * t2301 * t2302 - t974 * t2315;
    let t2318 = t2317 * t275;
    let t2320 = t176 * t2318 * sigma0;
    (t2317, t2320)
}

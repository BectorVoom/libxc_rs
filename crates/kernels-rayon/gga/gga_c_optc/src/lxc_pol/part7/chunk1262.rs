//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1262/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1262(t2367: f64, t7267: f64, t999: f64, t23574: f64, t2360: f64, t24386: f64, t24388: f64, t24392: f64, t24693: f64, t24696: f64, t24702: f64, t2563: f64, t26057: f64, t277: f64, t7268: f64, t7301: f64, t7304: f64, t914: f64, t95: f64, t962: f64) -> f64 {
    let t26063 = t999 * t2367 * t7267;
    let t26071 = -16.0_f64 / 9.0_f64 * t24386 - 64.0_f64 / 27.0_f64 * t24388 + 140.0_f64 / 81.0_f64 * t999 * t914 * t24392 * t23574 + 0.25844881434903430496e-2_f64 * t95 * t277 * t26057 * t962 + 4.0_f64 / 3.0_f64 * t26063 + 32.0_f64 / 3.0_f64 * t7304 * t2563 + 4.0_f64 * t2360 * t7268 - t24693 - 16.0_f64 / 3.0_f64 * t2360 * t7301 - t24696 - t24702;
    t26071
}

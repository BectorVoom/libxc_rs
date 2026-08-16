//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 666/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk666(t1200: f64, t1565: f64, t2886: f64, t4249: f64, t485: f64, t5454: f64, t5458: f64, t5469: f64, t275: f64, t176: f64, sigma2: f64) -> (f64, f64) {
    let t5471 = -t1200 * t5469 - 2.0_f64 * t4249 * t1565 + 2.0_f64 * t2886 * t5458 + t5454 * t485;
    let t5472 = t5471 * t275;
    let t5474 = t176 * t5472 * sigma2;
    (t5471, t5474)
}

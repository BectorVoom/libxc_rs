//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 478/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk478(t2293: f64, t569: f64, t568: f64, t600: f64, t524: f64, t894: f64) -> (f64, f64, f64, f64, f64) {
    let t2427 = t569 * t2293;
    let t2428 = t568 * t2427;
    let t2433 = t600 * t2293;
    let t2434 = t568 * t2433;
    let t2437 = t524 * t894;
    (t2427, t2428, t2433, t2434, t2437)
}

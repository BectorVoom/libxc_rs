//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 447/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk447(t2481: f64, t2482: f64, t2484: f64, t2489: f64, t2494: f64, t2499: f64, t2503: f64, t2508: f64, t2512: f64, t462: f64, t92: f64, t734: f64, t91: f64) -> (f64, f64) {
    let t2514 = t2481 + 2.0_f64 / 9.0_f64 * t2482 + 2.0_f64 / 3.0_f64 * t2484 - 2.0_f64 / 9.0_f64 * t462 * t2489 + 2.0_f64 / 3.0_f64 * t462 * t2494 + 2.0_f64 / 3.0_f64 * t462 * t2499 - t462 * t2503 / 3.0_f64 + 2.0_f64 * t92 * t2508 - t92 * t2512;
    let t2516 = t91 * t734 * t2514;
    (t2514, t2516)
}

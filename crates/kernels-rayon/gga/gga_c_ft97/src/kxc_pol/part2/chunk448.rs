//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 448/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk448(t2334: f64, t2338: f64, t2341: f64, t2352: f64, t2357: f64, t2364: f64, t2368: f64, t2376: f64, t2462: f64, t2478: f64, t2516: f64) -> (f64, f64) {
    let t2518 = 4.0_f64 / 9.0_f64 * t2334;
    let t2526 = -t2478 / 4.0_f64 + t2516 / 2.0_f64 + t2518 + 2.0_f64 / 9.0_f64 * t2338 + 2.0_f64 / 3.0_f64 * t2341 - 2.0_f64 / 9.0_f64 * t2352 + 2.0_f64 / 3.0_f64 * t2357 + 2.0_f64 / 3.0_f64 * t2364 - t2368 / 3.0_f64 + 2.0_f64 * t2376 - t2462;
    (t2518, t2526)
}

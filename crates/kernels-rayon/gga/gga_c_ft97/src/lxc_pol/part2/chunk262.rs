//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 262/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk262(t463: f64, t958: f64, t469: f64, t942: f64, t24: f64, t460: f64, t462: f64, t92: f64, t457: f64, t91: f64, t477: f64, t923: f64, t945: f64) -> (f64, f64, f64, f64, f64) {
    let t959 = t463 * t958;
    let t962 = t469 * t942;
    let t963 = t24 * t962;
    let t965 = -t460 - t462 * t959 / 3.0_f64 - t92 * t963;
    let t967 = t91 * t457 * t965;
    let t971 = t967 / 6.0_f64 - t477 - t923 / 9.0_f64 - t945 / 3.0_f64;
    (t959, t963, t965, t967, t971)
}

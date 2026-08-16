//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 184/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk184(t24: f64, t558: f64, t586: f64, t462: f64, t581: f64, t583: f64, t92: f64, t579: f64, t91: f64, t517: f64, t522: f64, t561: f64) -> (f64, f64, f64, f64, f64) {
    let t588 = t24 * t586 * t558;
    let t590 = -t581 - t462 * t583 / 3.0_f64 - t92 * t588;
    let t592 = t91 * t579 * t590;
    let t594 = t517 / 9.0_f64;
    let t597 = t592 / 6.0_f64 - t594 - t522 / 9.0_f64 - t561 / 3.0_f64;
    (t588, t590, t592, t594, t597)
}

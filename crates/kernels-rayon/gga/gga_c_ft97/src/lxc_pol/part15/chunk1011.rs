//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1011/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1011(t4449: f64, t4466: f64, t20049: f64, t930: f64, t4491: f64, t19978: f64, t938: f64, t1594: f64, t1624: f64, t1631: f64, t20050: f64, t20090: f64, t372: f64, t374: f64, t37835: f64, t4467: f64, t534: f64, t7906: f64, t7914: f64) -> (f64, f64, f64) {
    let t85608 = t4449 * t4466;
    let t85618 = t930 * t20049;
    let t85626 = t4449 * t4491;
    let t85630 = t19978 * t938;
    let t85644 = 0.279058811357253504e0_f64 * t37835 * t374 * t930 * t20090 - 0.19352371901929178119e-4_f64 * t372 * t534 * t85608 - 0.1674352868143521024e-1_f64 * t372 * t7914 * t85608 - 0.69716604262587839785e-3_f64 * t372 * t7906 * t85608 + 0.93019603785751168e-2_f64 * t372 * t1631 * t85618 + 0.69764702839313376e-1_f64 * t1624 * t374 * t4467 * t4491 - 0.11619434043764639964e-2_f64 * t1624 * t1594 * t85626 + 0.46477736175058559857e-3_f64 * t1624 * t7906 * t85630 + 0.12901581267952785412e-4_f64 * t1624 * t534 * t85630 - 0.139529405678626752e-1_f64 * t1624 * t1631 * t85626 + 0.46509801892875584e-1_f64 * t1624 * t374 * t20050 * t938;
    (t85618, t85630, t85644)
}

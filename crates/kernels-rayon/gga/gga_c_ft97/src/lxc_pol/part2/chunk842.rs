//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 842/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk842(t2190: f64, t925: f64, t9144: f64, t167: f64, t2101: f64, t12338: f64, t9114: f64, t12590: f64, t3455: f64, t379: f64, t2179: f64, t582: f64) -> (f64, f64, f64, f64, f64) {
    let t13204 = t925 * t2190;
    let t13205 = t9144 * t13204;
    let t13208 = t2101 * t167;
    let t13209 = t13208 * t12338;
    let t13212 = t9114 * t167;
    let t13213 = t13212 * t12590;
    let t13216 = t3455 * t379;
    let t13217 = t9144 * t13216;
    let t13220 = t582 * t2179;
    (t13205, t13209, t13213, t13217, t13220)
}

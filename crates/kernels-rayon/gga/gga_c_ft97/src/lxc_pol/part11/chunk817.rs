//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 817/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk817(t157: f64, t526: f64, t2178: f64, t358: f64, t167: f64, t2101: f64, t9114: f64, t2179: f64, t582: f64, t184: f64, t363: f64, t2: f64, t9952: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13140 = t526 * t157;
    let t13165 = t2178 * t358;
    let t13208 = t2101 * t167;
    let t13212 = t9114 * t167;
    let t13220 = t582 * t2179;
    let t13255 = t184 * t363;
    let t13313 = t9952 * t2;
    (t13140, t13165, t13208, t13212, t13220, t13255, t13313)
}

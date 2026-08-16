//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 531/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk531(t3271: f64, t452: f64, t488: f64, t447: f64, t499: f64, t925: f64, t2998: f64, t443: f64, t444: f64) -> (f64, f64, f64) {
    let t3273 = t452 * t488 * t3271;
    let t3277 = t447 * t499 * t925;
    let t3281 = t443 * t444 * t2998;
    (t3273, t3277, t3281)
}

//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 117/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk117(t321: f64, t332: f64, t2: f64, t3: f64, t8: f64, t39: f64) -> (f64, f64, f64) {
    let t333 = t321 * t332;
    let t339 = 1.0_f64 / t2 * t3;
    let t340 = 1.0_f64 / t8;
    let t341 = t39 * t340;
    (t333, t339, t341)
}

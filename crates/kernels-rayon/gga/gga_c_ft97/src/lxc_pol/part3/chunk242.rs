//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 242/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk242(t292: f64, t817: f64, t820: f64, t285: f64, t800: f64, t812: f64) -> (f64, f64) {
    let t293 = 0.1e-59_f64 < t292;
    let t821 = t817 * t820;
    let t824 = piecewise3(t293, -t285 * t821 + 2.0_f64 * t800 * t812, 0.0_f64);
    (t821, t824)
}

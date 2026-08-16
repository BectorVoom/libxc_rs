//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 95/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk95(t2: f64, t241: f64, t192: f64, t92: f64, t91: f64, t244: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t248 = t241 * t2;
    let t249 = t192 * t248;
    let t250 = t92 * t249;
    let t251 = f64::sqrt(t250);
    let t252 = t91 * t251;
    let t255 = 3.0_f64 + t252 / 3.0_f64 + t244 / 3.0_f64;
    (t248, t249, t250, t251, t252, t255)
}

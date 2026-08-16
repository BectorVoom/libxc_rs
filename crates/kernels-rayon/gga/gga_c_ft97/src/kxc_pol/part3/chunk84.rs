//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 84/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk84(t215: f64, t206: f64, t214: f64, t52: f64, t204: f64, t209: f64, t41: f64, t203: f64, rho1: f64) -> (f64, f64, f64, f64, f64) {
    let t216 = t215 * rho1;
    let t218 = 1.0_f64 / t206 / t216;
    let t220 = t52 * t214 * t218;
    let t221 = 0.55569193573523559258e-3_f64 * t220;
    let t222 = 1.0_f64 + 0.45058854638888888889e-1_f64 * t41 * t204 * t209 + t221;
    let t223 = t222 * t222;
    let t224 = t203 * t223;
    (t220, t221, t222, t223, t224)
}

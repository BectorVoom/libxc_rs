//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1041/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1041(t59339: f64, t73256: f64, t73259: f64, t73262: f64, t73276: f64, t73299: f64, t73301: f64, t86175: f64, t86178: f64, t86181: f64, t86188: f64, t86195: f64, t86199: f64) -> f64 {
    let t86440 = -2.0_f64 * t86175 + 4.0_f64 / 9.0_f64 * t86178 + 4.0_f64 / 3.0_f64 * t86181 - 2.0_f64 / 9.0_f64 * t73256 + 4.0_f64 / 9.0_f64 * t73259 - 8.0_f64 / 27.0_f64 * t73262 + 2.0_f64 / 27.0_f64 * t73276 + 4.0_f64 / 9.0_f64 * t86188 + t59339 + 4.0_f64 / 9.0_f64 * t73299 + 4.0_f64 / 9.0_f64 * t73301 - 4.0_f64 / 3.0_f64 * t86195 - 4.0_f64 / 3.0_f64 * t86199;
    t86440
}

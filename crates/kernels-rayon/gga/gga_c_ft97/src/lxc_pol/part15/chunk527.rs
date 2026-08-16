//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 527/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk527(t5299: f64, t799: f64, t27: f64, t89: f64, t2653: f64, t4230: f64, t4235: f64, t5211: f64, t5215: f64, t5219: f64, t5223: f64, t5228: f64) -> (f64, f64, f64) {
    let t5300 = t799 * t5299;
    let t5302 = t89 * t27 * t5300;
    let t5304 = t2653 + t4230 + t4235 - t5211 / 27.0_f64 + t5215 / 9.0_f64 + t5219 / 9.0_f64 - t5223 / 18.0_f64 + t5228 / 3.0_f64 - t5302 / 6.0_f64;
    (t5300, t5302, t5304)
}

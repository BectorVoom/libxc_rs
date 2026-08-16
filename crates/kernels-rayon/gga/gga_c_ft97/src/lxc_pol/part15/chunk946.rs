//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 946/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk946(t21052: f64, t8675: f64, t21056: f64, t21075: f64, t21068: f64, t21064: f64, t21059: f64, t21062: f64, t2253: f64, t21031: f64, t21040: f64, t21044: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t76056 = t8675 * t21052;
    let t76062 = t8675 * t21056;
    let t76101 = t8675 * t21075;
    let t76126 = t8675 * t21068;
    let t76128 = t8675 * t21064;
    let t76130 = t8675 * t21059;
    let t76199 = t2253 * t21062;
    let t76210 = t2253 * t21031;
    let t76221 = t2253 * t21040;
    let t76232 = t2253 * t21044;
    (t76056, t76062, t76101, t76126, t76128, t76130, t76199, t76210, t76221, t76232)
}

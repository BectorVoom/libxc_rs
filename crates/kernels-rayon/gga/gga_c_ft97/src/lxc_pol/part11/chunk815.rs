//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 815/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk815(t135: f64, t3347: f64, t131: f64, t538: f64, t120: f64, t1595: f64, t528: f64, t167: f64, t9132: f64, t582: f64, t605: f64, t2097: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12374 = t3347 * t135;
    let t12411 = t538 * t131;
    let t12488 = t1595 * t528 * t120;
    let t12703 = t9132 * t167;
    let t12709 = t582 * t605;
    let t12714 = t2097 * t605;
    (t12374, t12411, t12488, t12703, t12709, t12714)
}

//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1030/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1030(t41512: f64, t39370: f64, t420: f64, t701: f64, t704: f64, t2248: f64, t705: f64, t2451: f64, t626: f64, t2442: f64, t173: f64, t9653: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41513 = 0.18916624705075445817e-1_f64 * t41512;
    let t41516 = t701 * t420 * t704 * t39370;
    let t41519 = t701 * t2248 * t705;
    let t41522 = t701 * t626 * t2451;
    let t41525 = t701 * t626 * t2442;
    let t41528 = t701 * t173 * t9653;
    (t41513, t41516, t41519, t41522, t41525, t41528)
}

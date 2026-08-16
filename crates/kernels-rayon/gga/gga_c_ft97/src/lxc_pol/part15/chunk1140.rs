//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1140/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1140(t41911: f64, t41912: f64, t88252: f64, t89: f64, t27: f64, t676: f64, t88939: f64, t2372: f64, t88289: f64, t41848: f64, t88294: f64, t666: f64, t669: f64, t86571: f64) -> (f64, f64, f64, f64, f64) {
    let t89047 = t89 * t41911 * t41912 * t88252;
    let t89051 = t89 * t27 * t676 * t88939;
    let t89054 = t89 * t27 * t2372 * t88289;
    let t89058 = t89 * t27 * t41848 * t88294;
    let t89062 = t89 * t666 * t669 * t86571;
    (t89047, t89051, t89054, t89058, t89062)
}

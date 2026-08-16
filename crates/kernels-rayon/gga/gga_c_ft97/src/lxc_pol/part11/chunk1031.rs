//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1031/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1031(t173: f64, t701: f64, t9673: f64, t191: f64, t2347: f64, t2346: f64) -> (f64, f64, f64) {
    let t41531 = t701 * t173 * t9673;
    let t41534 = 1.0_f64 / t191 / t2347;
    let t41535 = t2346 * t2346;
    let t41536 = 1.0_f64 / t41535;
    (t41531, t41534, t41536)
}

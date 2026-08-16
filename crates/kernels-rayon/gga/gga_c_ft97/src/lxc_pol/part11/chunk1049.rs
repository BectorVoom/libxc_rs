//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1049/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1049(t2354: f64, t41833: f64, t446: f64, t2373: f64, t2413: f64, t9770: f64, t2459: f64, t2372: f64, t27: f64, t89: f64, t375: f64, t9709: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41835 = t446 * t2354 * t41833;
    let t41837 = t2413 * t2373;
    let t41839 = t446 * t9770 * t41837;
    let t41841 = t2459 * t2459;
    let t41844 = t89 * t27 * t2372 * t41841;
    let t41846 = t89 * t375 * t9709;
    (t41835, t41837, t41839, t41841, t41844, t41846)
}

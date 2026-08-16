//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1043/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1043(t11401: f64, t191: f64, t26: f64, t458: f64, t9573: f64, t9597: f64, t2360: f64, t322: f64, t17: f64, t41448: f64, t41536: f64, t92: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41743 = t11401 * t191;
    let t41744 = t26 * t41743;
    let t41745 = 280.0_f64 / 81.0_f64 * t41744;
    let t41746 = t458 * t9573;
    let t41748 = t458 * t9597;
    let t41751 = 1.0_f64 / t322 / t2360;
    let t41752 = t17 * t41751;
    let t41753 = t41536 * t41448;
    let t41755 = t92 * t41752 * t41753;
    (t41743, t41744, t41745, t41746, t41748, t41751, t41752, t41753, t41755)
}

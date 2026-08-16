//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2397/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2397(t47681: f64, t47686: f64, t47691: f64, t47695: f64, t47699: f64, t47703: f64, t47705: f64, t48085: f64, t48087: f64, t48090: f64, t48092: f64, t48096: f64) -> (f64, f64) {
    let t49127 = -0.99342e0_f64 * t48085 + 0.99342e0_f64 * t48087 + 0.49671e0_f64 * t48090 - 0.82785e-1_f64 * t48092 - 0.89459259259259259259e0_f64 * t47681 + 0.36230999999999999999e1_f64 * t47686 - 0.60384999999999999999e0_f64 * t47691 - 0.60384999999999999999e0_f64 * t47695 - 0.20128333333333333333e0_f64 * t47699 - 0.543465e1_f64 * t47703 + 0.80513333333333333334e0_f64 * t47705;
    let t49139 = 0.27595e0_f64 * t48096;
    (t49127, t49139)
}

//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1071/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1071(t2: f64, t33300: f64, t458: f64, t9965: f64, t13682: f64, t13683: f64, t192: f64, t2506: f64, t3917: f64, t41482: f64, t41794: f64, t41837: f64, t41841: f64, t41849: f64, t42059: f64, t42192: f64, t42194: f64, t42206: f64, t42207: f64, t42212: f64, t42214: f64, t42216: f64, t462: f64, t743: f64, t92: f64, t9896: f64) -> f64 {
    let t42218 = t33300 * t2;
    let t42227 = t458 * t9965;
    let t42229 = 8.0_f64 / 3.0_f64 * t42192 + 16.0_f64 / 9.0_f64 * t42194 + 8.0_f64 / 3.0_f64 * t13682 * t13683 * t42059 - 12.0_f64 * t462 * t3917 * t41482 - 4.0_f64 * t462 * t9896 * t41837 + t42206 + 112.0_f64 / 27.0_f64 * t42207 - t92 * t192 * t743 * t41794 + 16.0_f64 / 3.0_f64 * t42212 - 8.0_f64 / 3.0_f64 * t42214 + 8.0_f64 * t42216 + 24.0_f64 * t92 * t192 * t42218 * t41849 + 6.0_f64 * t92 * t192 * t2506 * t41841 + 4.0_f64 / 3.0_f64 * t42227;
    t42229
}

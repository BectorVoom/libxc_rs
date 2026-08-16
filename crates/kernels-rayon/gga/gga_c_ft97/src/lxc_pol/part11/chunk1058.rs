//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1058/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1058(t41957: f64, t89: f64, t9725: f64, t9750: f64, t375: f64, t9567: f64, t9718: f64, t241: f64, t41446: f64, t41448: f64, t9716: f64, t39370: f64, t666: f64, t669: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41958 = 8.0_f64 / 81.0_f64 * t41957;
    let t41960 = t89 * t9725 * t9750;
    let t41962 = t375 * t9567;
    let t41964 = t89 * t41962 * t9718;
    let t41966 = t241 * t41446;
    let t41969 = t89 * t9716 * t41966 * t41448;
    let t41973 = t89 * t666 * t669 * t39370;
    (t41958, t41960, t41962, t41964, t41969, t41973)
}

//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1011/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1011(t1882: f64, t9402: f64, t157: f64, t40424: f64, t8392: f64, t9100: f64, t2144: f64, t8232: f64, t376: f64, t89: f64, t9396: f64, t605: f64, t9114: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41246 = t1882 * t9402;
    let t41251 = t40424 * t157;
    let t41262 = t8392 * t9100;
    let t41264 = t8232 * t2144;
    let t41267 = t89 * t376 * t9396;
    let t41269 = t9114 * t605;
    (t41246, t41251, t41262, t41264, t41267, t41269)
}

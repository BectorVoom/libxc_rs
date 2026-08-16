//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1053/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1053(t2345: f64, t2348: f64, t41468: f64, t89: f64, t2362: f64, t9733: f64, t2336: f64, t9737: f64, t41448: f64, t666: f64, t9749: f64, t2361: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41895 = t89 * t2345 * t2348 * t41468;
    let t41898 = t89 * t9733 * t2362;
    let t41899 = 8.0_f64 / 27.0_f64 * t41898;
    let t41901 = t89 * t2336 * t9737;
    let t41905 = t89 * t666 * t9749 * t41448;
    let t41909 = t89 * t666 * t2361 * t41468;
    (t41895, t41898, t41899, t41901, t41905, t41909)
}

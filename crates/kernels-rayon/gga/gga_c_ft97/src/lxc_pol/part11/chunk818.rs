//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 818/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk818(t122: f64, t695: f64, t677: f64, t25: f64, t200: f64, t709: f64, t807: f64, t9542: f64, t9524: f64, t173: f64, t2440: f64, t420: f64, t9651: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13467 = t695 * t122;
    let t13468 = t677 * t13467;
    let t13473 = t695 * t25;
    let t13474 = t677 * t13473;
    let t13521 = t200 * t709;
    let t13531 = t807 * t9542;
    let t13589 = t9524 * t9542;
    let t13598 = t173 * t2440;
    let t13605 = t420 * t9651;
    (t13468, t13474, t13521, t13531, t13589, t13598, t13605)
}

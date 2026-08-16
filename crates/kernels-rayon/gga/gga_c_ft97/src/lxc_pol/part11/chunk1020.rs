//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1020/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1020(t41435: f64, t446: f64, t9744: f64, t1882: f64, t9772: f64, t10039: f64, t684: f64, t9770: f64, t2346: f64, t2359: f64) -> (f64, f64, f64, f64, f64) {
    let t41437 = t446 * t9744 * t41435;
    let t41439 = t1882 * t9772;
    let t41441 = t10039 * t684;
    let t41443 = t446 * t9770 * t41441;
    let t41446 = 1.0_f64 / t2346 / t2359;
    (t41437, t41439, t41441, t41443, t41446)
}

//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 976/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk976(t21744: f64, t8392: f64, t1882: f64, t21724: f64, t13598: f64, t1526: f64, t21103: f64, t4922: f64, t9483: f64, t21114: f64, t21110: f64, t21118: f64, t342: f64, t630: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t81730 = t8392 * t21744;
    let t81780 = t1882 * t21724;
    let t81955 = t1526 * t13598 * t21103;
    let t81958 = t1526 * t9483 * t4922;
    let t81968 = t1526 * t9483 * t21114;
    let t81971 = t1526 * t9483 * t21110;
    let t81974 = t342 * t630 * t21118;
    (t81730, t81780, t81955, t81958, t81968, t81971, t81974)
}

//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 370/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk370(t668: f64, t761: f64, t342: f64, t630: f64, t784: f64, t294: f64, t10: f64, t1542: f64, t296: f64, t2347: f64, t295: f64, t683: f64, t798: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2607 = t761 * t668;
    let t2638 = t342 * t630 * t784 / 12.0_f64;
    let t2639 = t294 * t668;
    let t2652 = t10 * t1542 * t296;
    let t2653 = 2.0_f64 / 27.0_f64 * t2652;
    let t2660 = t295 * t2347;
    let t2665 = t683 * t798;
    (t2607, t2638, t2639, t2652, t2653, t2660, t2665)
}

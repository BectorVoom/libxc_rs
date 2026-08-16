//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 547/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk547(t3491: f64, t590: f64, t91: f64, t1033: f64, t1775: f64, t2: f64, t2097: f64, t2984: f64, t2102: f64, t3323: f64, t582: f64, t2993: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3493 = t91 * t3491 * t590;
    let t3497 = t1775 * t1033;
    let t3499 = t2097 * t2;
    let t3500 = t3499 * t2984;
    let t3503 = t2102 * t3323;
    let t3506 = t582 * t2;
    let t3507 = t3506 * t2993;
    (t3493, t3497, t3499, t3500, t3503, t3506, t3507)
}

//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 852/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk852(t16745: f64, t16748: f64, t16751: f64, t2: f64, t4714: f64, t1985: f64, t558: f64, t4668: f64, t9016: f64, t3408: f64, t3518: f64, t16395: f64, t582: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17249 = t16745 / 9.0_f64;
    let t17250 = 2.0_f64 / 9.0_f64 * t16748;
    let t17251 = 2.0_f64 / 27.0_f64 * t16751;
    let t17254 = t2 * t4714;
    let t17256 = t1985 * t17254 * t558;
    let t17259 = t2 * t4668;
    let t17261 = t9016 * t17259 * t558;
    let t17265 = t1985 * t3518 * t3408;
    let t17268 = t582 * t16395;
    (t17249, t17250, t17251, t17256, t17261, t17265, t17268)
}

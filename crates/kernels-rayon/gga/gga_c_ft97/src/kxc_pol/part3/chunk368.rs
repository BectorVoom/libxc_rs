//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 368/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk368(t1570: f64, t179: f64, t178: f64, t1638: f64, t342: f64, t630: f64, t657: f64, t420: f64, t703: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2271 = t179 * t1570;
    let t2280 = t178 * t178;
    let t2281 = 1.0_f64 / t2280;
    let t2289 = 0.19257444444444444444e0_f64 * t1638;
    let t2319 = t342 * t630 * t657 / 12.0_f64;
    let t2320 = t420 * t703;
    (t2271, t2280, t2281, t2289, t2319, t2320)
}

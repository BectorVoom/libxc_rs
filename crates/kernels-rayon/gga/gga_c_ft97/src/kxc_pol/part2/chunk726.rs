//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 726/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk726(t11032: f64, t11418: f64, t348: f64, t1537: f64, t3108: f64, t7733: f64, t947: f64, t3196: f64, t8392: f64, t1647: f64, t3182: f64, t1909: f64) -> (f64, f64, f64, f64, f64) {
    let t11419 = t11032 + t11418;
    let t11420 = t348 * t11419;
    let t11424 = t1537 * t3108;
    let t11427 = t7733 * t947;
    let t11430 = 4.0_f64 / 81.0_f64 * t8392 * t3196;
    let t11431 = t3182 * t1647;
    let t11432 = t1909 * t11431;
    (t11420, t11424, t11427, t11430, t11432)
}

//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 919/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk919(t1131: f64, t2569: f64, t2568: f64, t729: f64, t1882: f64, t3848: f64, t1170: f64, t8232: f64, t3953: f64, t681: f64, t89: f64, t2469: f64, t3859: f64) -> (f64, f64, f64, f64, f64) {
    let t14226 = t1131 * t2569;
    let t14228 = t729 * t2568 * t14226;
    let t14232 = 2.0_f64 / 27.0_f64 * t1882 * t3848;
    let t14233 = t8232 * t1170;
    let t14240 = 2.0_f64 / 9.0_f64 * t89 * t681 * t3953;
    let t14242 = t729 * t2469 * t3859;
    (t14228, t14232, t14233, t14240, t14242)
}

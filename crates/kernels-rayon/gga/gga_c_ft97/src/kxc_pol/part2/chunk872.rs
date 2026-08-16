//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 872/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk872(t9524: f64, t9542: f64, t13346: f64, t2320: f64, t701: f64, t3700: f64, t9483: f64, t173: f64, t2440: f64, t3691: f64, t13309: f64, t3806: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13589 = t9524 * t9542;
    let t13592 = t2320 * t13346;
    let t13593 = t701 * t13592;
    let t13595 = t9483 * t3700;
    let t13596 = t701 * t13595;
    let t13598 = t173 * t2440;
    let t13599 = t13598 * t3691;
    let t13600 = t701 * t13599;
    let t13601 = 0.56749874115226337448e-2_f64 * t13600;
    let t13602 = t3806 * t13309;
    (t13589, t13593, t13596, t13600, t13601, t13602)
}

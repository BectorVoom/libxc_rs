//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 984/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk984(t1775: f64, t22326: f64, t22316: f64, t22321: f64, t22319: f64, t22302: f64, t22298: f64, t22313: f64, t22284: f64, t21985: f64, t2336: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t83385 = t1775 * t22326;
    let t83387 = t1775 * t22316;
    let t83410 = t1775 * t22321;
    let t83463 = t1775 * t22319;
    let t83472 = t1775 * t22302;
    let t83474 = t1775 * t22298;
    let t83569 = t1775 * t22313;
    let t83587 = t1775 * t22284;
    let t83606 = t89 * t2336 * t21985;
    (t83385, t83387, t83410, t83463, t83472, t83474, t83569, t83587, t83606)
}

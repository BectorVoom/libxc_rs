//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 664/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk664(t1526: f64, t2322: f64, t9483: f64, t2326: f64, t342: f64, t630: f64, t194: f64, t196: f64, t122: f64, t2427: f64, t677: f64, t3724: f64, t694: f64, t709: f64) -> (f64, f64, f64, f64, f64) {
    let t9485 = t1526 * t9483 * t2322;
    let t9488 = t342 * t630 * t2326;
    let t9523 = 1.0_f64 / t196 / t194;
    let t9524 = t122 * t9523;
    let t9533 = t677 * t2427;
    let t9545 = t3724 * t694 * t709;
    (t9485, t9488, t9524, t9533, t9545)
}

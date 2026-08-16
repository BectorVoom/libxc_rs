//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 465/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk465(t3188: f64, t3440: f64, t3439: f64, t1017: f64, t160: f64, t379: f64, t2221: f64, t558: f64, t167: f64, t2185: f64, t609: f64, t574: f64, t605: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3441 = t3440 * t3188;
    let t3442 = t3439 * t3441;
    let t3445 = t160 * t1017;
    let t3446 = t3445 * t379;
    let t3447 = t2221 * t3446;
    let t3450 = t1017 * t558;
    let t3452 = t2185 * t167 * t3450;
    let t3455 = t1017 * t609;
    let t3457 = t574 * t605 * t3455;
    (t3441, t3442, t3446, t3447, t3450, t3452, t3455, t3457)
}

//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 473/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk473(t3565: f64, t605: f64, t144: f64, t1060: f64, t558: f64, t574: f64, t1017: f64, t616: f64, t1045: f64, t604: f64) -> (f64, f64, f64, f64, f64) {
    let t3566 = t605 * t3565;
    let t3567 = t144 * t3566;
    let t3571 = t574 * t1060 * t558;
    let t3575 = t574 * t616 * t1017;
    let t3578 = t1045 * t604;
    (t3566, t3567, t3571, t3575, t3578)
}

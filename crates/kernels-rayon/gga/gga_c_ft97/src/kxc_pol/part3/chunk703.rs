//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 703/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk703(t582: f64, t605: f64, t2097: f64, t157: f64, t9224: f64, t160: f64, t7763: f64, t7800: f64, t1047: f64, t1637: f64, t89: f64, t1570: f64, t586: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12709 = t582 * t605;
    let t12714 = t2097 * t605;
    let t12723 = t9224 * t157;
    let t12724 = t160 * t7763;
    let t12746 = t160 * t7800;
    let t12752 = t89 * t1637 * t1047;
    let t12791 = t586 * t1570;
    (t12709, t12714, t12723, t12724, t12746, t12752, t12791)
}

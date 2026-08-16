//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 175/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk175(t139: f64, t538: f64, t527: f64, t129: f64, t131: f64, t137: f64) -> (f64, f64, f64, f64, f64) {
    let t539 = t139 * t538;
    let t540 = t527 * t539;
    let t542 = t129 * t131;
    let t543 = t542 * t139;
    let t548 = t137 * t137;
    let t549 = 1.0_f64 / t548;
    (t540, t542, t543, t548, t549)
}

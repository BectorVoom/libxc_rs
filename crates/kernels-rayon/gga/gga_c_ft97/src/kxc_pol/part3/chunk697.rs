//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 697/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk697(t2253: f64, t3642: f64, t1736: f64, t179: f64, t3627: f64, t41: f64, t70: f64, t3618: f64, t8675: f64, t3622: f64, t1068: f64, t8640: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12132 = 2.0_f64 * t2253 * t3642;
    let t12137 = t1736 * t179;
    let t12143 = t41 * t3627 * t70;
    let t12162 = 4.0_f64 / 9.0_f64 * t8675 * t3618;
    let t12164 = 4.0_f64 / 9.0_f64 * t8675 * t3622;
    let t12165 = t8640 * t1068;
    (t12132, t12137, t12143, t12162, t12164, t12165)
}

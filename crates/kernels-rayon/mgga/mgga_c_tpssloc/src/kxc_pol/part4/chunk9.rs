//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 9/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk9(t11: f64, t17: f64, t19: f64, t21: f64, t9: f64, t5: f64) -> (f64, f64) {
    let t24 = 0.35e0_f64 + 0.87e0_f64 * t9 * t11 + 0.5e0_f64 * t17 + 0.226e1_f64 * t19 * t21;
    let t25 = 1.0_f64 + t5;
    (t24, t25)
}

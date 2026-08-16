//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 197/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk197(t576: f64, t577: f64, t11: f64, t2: f64, t10: f64, t3: f64) -> (f64, f64, f64, f64, f64) {
    let t580 = 1.0_f64 + 0.45e1_f64 * t576 * t577;
    let t581 = t2 * t11;
    let t582 = 0.174e1_f64 * t581;
    let t583 = t10 * t3;
    let t584 = 1.0_f64 / t583;
    (t580, t581, t582, t583, t584)
}

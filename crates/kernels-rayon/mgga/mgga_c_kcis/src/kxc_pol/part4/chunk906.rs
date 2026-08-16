//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 906/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk906(t20: f64, t2314: f64, t725: f64, t2316: f64, t2469: f64, t2: f64, t2456: f64, t647: f64, t649: f64, t691: f64, t3: f64, t8572: f64) -> (f64, f64, f64, f64, f64) {
    let t8578 = t2314 * t725 * t20;
    let t8581 = t2316 * t2469;
    let t8585 = t647 * t2456 * t2;
    let t8590 = t649 * t691;
    let t8593 = t8572 * t3;
    (t8578, t8581, t8585, t8590, t8593)
}

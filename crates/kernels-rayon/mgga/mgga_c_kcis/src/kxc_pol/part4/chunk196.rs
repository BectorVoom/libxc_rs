//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 196/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk196(t187: f64, t471: f64, t483: f64, t592: f64, t601: f64) -> f64 {
    let t609 = -t471 + t187 * (-0.3109e-1_f64 * t592 * t601 + t471 - 0.19751789702565206229e-1_f64 * t483) + 0.19751789702565206229e-1_f64 * t187 * t483;
    t609
}

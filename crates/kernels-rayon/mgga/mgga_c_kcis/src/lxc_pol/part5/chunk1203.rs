//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1203/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1203(t20018: f64, t20055: f64, t20093: f64, t20126: f64, t355: f64, t377: f64, t1175: f64, t6689: f64, t3464: f64, t14781: f64, t284: f64, t5048: f64, sigma0: f64) -> (f64, f64, f64) {
    let t20128 = t20018 + t20055 + t20093 + t20126;
    let t20129 = t20128 * t355;
    let t20130 = t20129 * sigma0;
    let t20131 = t20130 * t377;
    let t20133 = t1175 * t6689;
    let t20134 = t3464 * t20133;
    let t20136 = t14781 * t284;
    let t20137 = t20136 * t5048;
    (t20131, t20134, t20137)
}

//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1291/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1291(t102081: f64, t102085: f64, t102088: f64, t102092: f64, t27567: f64, t27583: f64, t29575: f64, t94928: f64, t94974: f64, t94977: f64, t99176: f64, t99193: f64, t99229: f64, t99238: f64) -> f64 {
    let t102098 = -0.15445601851851851852e-3_f64 * t99176 + t99193 - 0.46336805555555555556e-3_f64 * t27583 * t102081 + 0.25794135802469135802e-2_f64 * t102085 + 0.15459116753472222222e-4_f64 * t27567 * t102088 + 0.11584201388888888889e-3_f64 * t102092 + t99229 + 0.23168402777777777778e-3_f64 * t94928 * t29575 + t99238 - 0.7722800925925925926e-4_f64 * t94974 - 0.7722800925925925926e-4_f64 * t94977;
    t102098
}

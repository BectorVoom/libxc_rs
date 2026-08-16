//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3704/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3704(t56176: f64, t56183: f64, t56185: f64, t56187: f64, t56189: f64, t56209: f64, t56212: f64, t56214: f64, t56216: f64, t56228: f64, t68363: f64, t68366: f64) -> f64 {
    let t70186 = -0.23706666666666666667e0_f64 * t68363 + 0.65851851851851851853e-1_f64 * t68366 - 0.17560493827160493828e-1_f64 * t56176 + 0.52681481481481481483e-1_f64 * t56183 - 0.39511111111111111112e-1_f64 * t56185 - 0.19755555555555555556e-1_f64 * t56187 - 0.59266666666666666668e-1_f64 * t56189 + 0.13170370370370370371e-1_f64 * t56209 + 0.65851851851851851853e-2_f64 * t56212 + 0.39511111111111111112e-1_f64 * t56214 - 0.10975308641975308642e-1_f64 * t56216 + 0.26340740740740740742e-1_f64 * t56228;
    t70186
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3452/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3452(t41361: f64, t41363: f64, t51973: f64, t51978: f64, t63325: f64, t63328: f64, t63336: f64, t63338: f64, t63340: f64, t63342: f64, t63346: f64, t63351: f64, t63355: f64) -> f64 {
    let t64959 = -0.26340740740740740742e-1_f64 * t51973 + 0.30730864197530864199e-1_f64 * t51978 + 0.30730864197530864198e-1_f64 * t41361 + 0.13170370370370370371e-1_f64 * t41363 - 0.65851851851851851853e-1_f64 * t63325 + 0.23706666666666666667e0_f64 * t63328 + 0.35560000000000000001e0_f64 * t63336 - 0.39511111111111111112e-1_f64 * t63338 + 0.13170370370370370371e-1_f64 * t63340 + 0.10975308641975308642e-1_f64 * t63342 - 0.16462962962962962963e-1_f64 * t63346 - 0.43901234567901234568e-1_f64 * t63351 + 0.59266666666666666668e-1_f64 * t63355;
    t64959
}

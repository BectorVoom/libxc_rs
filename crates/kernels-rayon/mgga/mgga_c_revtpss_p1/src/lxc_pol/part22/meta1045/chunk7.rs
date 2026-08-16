//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3666/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3666(t43911: f64, t56176: f64, t56183: f64, t56185: f64, t68342: f64, t68347: f64, t68350: f64, t68353: f64, t68357: f64, t68360: f64, t68363: f64, t68366: f64, t68368: f64, t68370: f64, t68373: f64) -> f64 {
    let t69263 = 0.57386111111111111112e0_f64 * t68342 + 0.68863333333333333334e1_f64 * t68347 - 0.20659e1_f64 * t68350 - 0.123954e2_f64 * t68353 - 0.68863333333333333334e0_f64 * t68357 + 0.123954e2_f64 * t68360 - 0.82636000000000000001e1_f64 * t68363 + 0.22954444444444444444e1_f64 * t68366 - 0.27785333333333333334e0_f64 * t68368 - 0.61745185185185185186e-1_f64 * t68370 + 0.6311625e0_f64 * t68373 - 0.3859074074074074074e-1_f64 * t43911 - 0.6121185185185185185e0_f64 * t56176 + 0.18363555555555555555e1_f64 * t56183 - 0.13772666666666666666e1_f64 * t56185;
    t69263
}

//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 505/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk505(t60: f64, t50: f64, t990: f64, t1383: f64, t1386: f64, t154: f64, t441: f64, t5343: f64, t5512: f64, t814: f64, t922: f64, t925: f64, t5511: f64, zeta_threshold: f64) -> f64 {
    let t61 = t60 <= zeta_threshold;
    let t5515 = t990 * t50;
    let t5525 = piecewise3(t61, 0.0_f64, 8.0_f64 / 27.0_f64 * t5512 * t922 + 8.0_f64 / 9.0_f64 * t5515 * t5343 - 2.0_f64 / 9.0_f64 * t1383 * t925 - 4.0_f64 / 3.0_f64 * t441 * t814 + 4.0_f64 * t1386 * t154);
    let t5527 = t5511 / 2.0_f64 + t5525 / 2.0_f64;
    t5527
}

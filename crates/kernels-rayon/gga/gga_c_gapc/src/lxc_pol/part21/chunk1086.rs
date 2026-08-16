//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1086/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1086(t11991: f64, t33476: f64, t11994: f64, t11320: f64, t11938: f64, t928: f64, t33460: f64, t33462: f64, t33464: f64, t33466: f64, t33468: f64, t33470: f64, t33472: f64, t33474: f64) -> f64 {
    let t33477 = t33476 * t11991;
    let t33479 = t33476 * t11994;
    let t33482 = t928 * t11320 * t11938;
    let t33484 = -0.16882049790461501058e-6_f64 * t33460 - 0.22509399720615334744e-6_f64 * t33462 - 0.90579542097823505428e-7_f64 * t33464 + 0.35170937063461460536e-8_f64 * t33466 - 0.77294542590142724635e-6_f64 * t33468 + 0.1374296967252737644e-5_f64 * t33470 + 0.11254699860307667372e-6_f64 * t33472 - 0.2845640240200497334e-7_f64 * t33474 - 0.4637672555408563478e-4_f64 * t33477 + 0.4637672555408563478e-4_f64 * t33479 + 0.38647271295071362318e-6_f64 * t33482;
    t33484
}

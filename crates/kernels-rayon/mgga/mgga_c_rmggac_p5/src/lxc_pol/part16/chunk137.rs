//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 137/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk137(t361: f64, t380: f64, t383: f64, t387: f64, t390: f64, t423: f64, t425: f64, t430: f64, t435: f64, t195: f64) -> (f64, f64, f64) {
    let t449 = t361 + t380 + t383 - t387 + t390 + t423 + t425 - t430 - t435;
    let t452 = t195 * t195;
    let t453 = 1.0_f64 / t452;
    (t449, t452, t453)
}

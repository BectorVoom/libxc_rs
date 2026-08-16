//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 520/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk520(t196: f64, t5738: f64, t1001: f64, t4179: f64, t1023: f64, t4090: f64, t4324: f64, t4328: f64, t436: f64, t4361: f64, t4365: f64, t500: f64, t5445: f64, t5447: f64, t5449: f64, t5451: f64, t5452: f64, t5459: f64, t5460: f64, t5461: f64, t5464: f64, t5466: f64, t5468: f64, t5471: f64, t5527: f64, t619: f64) -> f64 {
    let t5739 = t196 * t5738;
    let t5744 = t4179 * t1001;
    let t5749 = t4361 - t4365 + t5445 + t5447 + t5449 - t5451 + 0.186546e0_f64 * t5452 * t1023 + 0.31091e-1_f64 * t5739 * t500 + 0.93273e-1_f64 * t436 * t5527 + t4324 - t5459 - t5460 - t5461 + t4328 + 0.62182e-1_f64 * t619 * t5744 - t5464 + t5466 - t5468 + t5471 - 0.31091e-1_f64 * t619 * t4090;
    t5749
}

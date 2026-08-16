//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 156/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk156(t361: f64, t380: f64, t383: f64, t387: f64, t390: f64, t423: f64, t425: f64, t430: f64, t435: f64, t436: f64, t446: f64, t499: f64, t500: f64) -> f64 {
    let t503 = t361 + t380 + t383 - t387 + t390 + t423 + t425 - t430 - t435 + 0.93273e-1_f64 * t436 * t446 + 0.31091e-1_f64 * t499 * t500;
    t503
}

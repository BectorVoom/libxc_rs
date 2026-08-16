//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1120/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1120(t10252: f64, t1923: f64, t38029: f64, t43861: f64, t43862: f64, t43864: f64, t47510: f64, t47512: f64, t47516: f64, t47520: f64, t47524: f64, t47528: f64, t47530: f64, t47532: f64, t47534: f64, t47536: f64, t47538: f64, t8048: f64, t9128: f64) -> f64 {
    let t49237 = -0.1276937996798935182e-3_f64 * t47510 + 0.19863479950205658386e-4_f64 * t47512 - 0.2363e1_f64 * t1923 * t8048 - 0.11974241701863808564e0_f64 * t9128 * t10252 - t43861 + t43862 - 0.85129199786595678799e-5_f64 * t47516 - 0.7661627980793611092e-4_f64 * t47520 + 0.10215503974391481456e-3_f64 * t47524 + 0.2553875993597870364e-4_f64 * t47528 - 0.2553875993597870364e-4_f64 * t47530 + 0.212822999466489197e-4_f64 * t47532 + 0.5107751987195740728e-4_f64 * t47534 - 0.5107751987195740728e-4_f64 * t47536 + 0.8980681276397856423e-1_f64 * t47538 - t43864 + t38029;
    t49237
}

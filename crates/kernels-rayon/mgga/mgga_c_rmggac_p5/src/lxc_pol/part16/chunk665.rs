//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 665/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk665(t352: f64, t9523: f64, t9001: f64, t9009: f64, t305: f64, t5148: f64, t8971: f64, t8973: f64, t8998: f64, t9003: f64, t9006: f64, t9011: f64, t9013: f64, t9015: f64, t9017: f64, t9021: f64, t9023: f64, t9383: f64) -> f64 {
    let t9577 = t9523 * t352;
    let t9583 = 0.15965655602485078085e0_f64 * t9001;
    let t9586 = 0.23948483403727617128e0_f64 * t9009;
    let t9593 = 0.5987120850931904282e-1_f64 * t8971 - 0.5987120850931904282e-1_f64 * t8973 - 0.11974241701863808564e0_f64 * t5148 * t9577 + 0.59871208509319042821e-1_f64 * t305 * t9383 + 0.79828278012425390427e-1_f64 * t8998 - t9583 + 0.5987120850931904282e-1_f64 * t9003 + 0.5987120850931904282e-1_f64 * t9006 + t9586 - 0.17961362552795712846e0_f64 * t9011 + 0.35922725105591425692e0_f64 * t9013 + 0.8980681276397856423e-1_f64 * t9015 - 0.17961362552795712846e0_f64 * t9017 - 0.5987120850931904282e-1_f64 * t9021 + 0.8980681276397856423e-1_f64 * t9023;
    t9593
}

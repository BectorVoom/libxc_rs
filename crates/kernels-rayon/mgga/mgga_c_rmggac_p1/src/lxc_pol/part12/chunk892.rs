//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 892/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk892(t2318: f64, t34976: f64, t39437: f64, t7455: f64, t1550: f64, t1624: f64, t2124: f64, t2402: f64, t34813: f64, t35053: f64, t35056: f64, t35058: f64, t39401: f64, t39403: f64, t39406: f64, t39418: f64, t39420: f64, t39423: f64, t39425: f64, t39427: f64, t39433: f64, t39435: f64, t5016: f64, t798: f64, t8371: f64, t903: f64) -> f64 {
    let t39440 = t39437 * t34976 * t2318 * t7455;
    let t39442 = -0.42564599893297839398e-5_f64 * t39401 - 0.85129199786595678796e-5_f64 * t39403 + t39406 + 0.35922725105591425692e0_f64 * t903 * t2402 * t798 - 0.23948483403727617128e0_f64 * t5016 * t8371 - 0.23948483403727617128e0_f64 * t1550 * t2124 * t1624 - 0.40650199722100037752e-3_f64 * t35053 - 0.81300399444200075504e-3_f64 * t35056 - 0.12769379967989351819e-4_f64 * t39418 + 0.25538759935978703638e-4_f64 * t39420 - 0.11918087970123395032e-3_f64 * t35058 + 0.17961362552795712846e0_f64 * t39423 + 0.5987120850931904282e-1_f64 * t39425 + 0.71845450211182851384e0_f64 * t34813 * t39427 - 0.15961724959986689774e-4_f64 * t39433 - 0.25538759935978703638e-4_f64 * t39435 - 0.23942587439980034662e-4_f64 * t39440;
    t39442
}

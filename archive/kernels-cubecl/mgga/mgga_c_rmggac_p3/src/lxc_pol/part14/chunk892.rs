//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 892/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk892<F: Float>(t2318: F, t34976: F, t39437: F, t7455: F, t1550: F, t1624: F, t2124: F, t2402: F, t34813: F, t35053: F, t35056: F, t35058: F, t39401: F, t39403: F, t39406: F, t39418: F, t39420: F, t39423: F, t39425: F, t39427: F, t39433: F, t39435: F, t5016: F, t798: F, t8371: F, t903: F) -> F {
    let t39440 = t39437 * t34976 * t2318 * t7455;
    let t39442 = -F::cast_from(0.42564599893297839398e-5_f64) * t39401 - F::cast_from(0.85129199786595678796e-5_f64) * t39403 + t39406 + F::cast_from(0.35922725105591425692e0_f64) * t903 * t2402 * t798 - F::cast_from(0.23948483403727617128e0_f64) * t5016 * t8371 - F::cast_from(0.23948483403727617128e0_f64) * t1550 * t2124 * t1624 - F::cast_from(0.40650199722100037752e-3_f64) * t35053 - F::cast_from(0.81300399444200075504e-3_f64) * t35056 - F::cast_from(0.12769379967989351819e-4_f64) * t39418 + F::cast_from(0.25538759935978703638e-4_f64) * t39420 - F::cast_from(0.11918087970123395032e-3_f64) * t35058 + F::cast_from(0.17961362552795712846e0_f64) * t39423 + F::cast_from(0.5987120850931904282e-1_f64) * t39425 + F::cast_from(0.71845450211182851384e0_f64) * t34813 * t39427 - F::cast_from(0.15961724959986689774e-4_f64) * t39433 - F::cast_from(0.25538759935978703638e-4_f64) * t39435 - F::cast_from(0.23942587439980034662e-4_f64) * t39440;
    t39442
}

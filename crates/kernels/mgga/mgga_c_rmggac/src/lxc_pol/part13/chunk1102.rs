//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1102/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1102<F: Float>(t42086: F, t42101: F, t2471: F, t839: F, t10820: F, t1364: F, t2463: F, t36902: F, t36906: F, t42066: F, t42068: F, t42071: F, t42076: F, t42081: F, t42083: F, t42091: F, t42093: F, t42099: F, t42109: F, t42114: F, t5752: F, t708: F) -> (F, F) {
    let t44004 = F::cast_from(0.39726959900411316772e-4_f64) * t42086;
    let t44008 = F::cast_from(0.11918087970123395032e-3_f64) * t42101;
    let t44011 = t2471 * t839;
    let t44016 = F::cast_from(0.17961362552795712846e0_f64) * t42066 - F::cast_from(0.71845450211182851384e0_f64) * t42068 - F::cast_from(0.35922725105591425692e0_f64) * t42071 - F::cast_from(0.11974241701863808564e0_f64) * t10820 * t2463 + F::cast_from(0.1915406995198402773e-3_f64) * t42076 - F::cast_from(0.19957069503106347607e-1_f64) * t5752 * t708 - F::cast_from(0.5987120850931904282e-1_f64) * t42081 + F::cast_from(0.212822999466489197e-4_f64) * t42083 + t44004 + F::cast_from(0.85129199786595678799e-5_f64) * t42091 + F::cast_from(0.1702583995731913576e-4_f64) * t42093 + F::cast_from(0.1702583995731913576e-4_f64) * t42099 - t44008 - F::cast_from(0.10215503974391481456e-3_f64) * t42109 - F::cast_from(0.5107751987195740728e-4_f64) * t42114 - F::cast_from(0.23948483403727617128e0_f64) * t1364 * t44011 + F::cast_from(0.72042316457491791901e-3_f64) * t36902 + F::cast_from(0.1440846329149835838e-2_f64) * t36906;
    (t44011, t44016)
}

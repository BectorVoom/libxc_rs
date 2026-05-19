//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 817/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk817<F: Float>(t38361: F, t38363: F, t38365: F, t38367: F, t38371: F, t38375: F, t38377: F, t38382: F, t38384: F, t38387: F, t38389: F, t38391: F, t38393: F, t38395: F, t38398: F, t38404: F, t4041: F, t884: F, t8960: F) -> F {
    let t38406 = F::cast_from(0.25538759935978703638e-4_f64) * t38361 + F::cast_from(0.25538759935978703638e-4_f64) * t38363 - F::cast_from(0.42564599893297839398e-5_f64) * t38365 - F::cast_from(0.85129199786595678796e-5_f64) * t38367 - F::cast_from(0.1064114997332445985e-4_f64) * t38371 - F::cast_from(0.1064114997332445985e-4_f64) * t38375 - F::cast_from(0.53205749866622299248e-5_f64) * t38377 + F::cast_from(0.11974241701863808564e0_f64) * t4041 * t8960 + F::cast_from(0.14635184302277988245e0_f64) * t38382 + F::cast_from(0.59871208509319042821e-1_f64) * t884 * t38384 + F::cast_from(0.85129199786595678796e-5_f64) * t38387 + F::cast_from(0.85129199786595678796e-5_f64) * t38389 - F::cast_from(0.25538759935978703638e-4_f64) * t38391 + F::cast_from(0.25538759935978703638e-4_f64) * t38393 + F::cast_from(0.85129199786595678796e-5_f64) * t38395 + F::cast_from(0.25538759935978703638e-4_f64) * t38398 + F::cast_from(0.12769379967989351819e-4_f64) * t38404;
    t38406
}

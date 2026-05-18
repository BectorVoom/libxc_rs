//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 817/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk817<F: Float>(t38361: F, t38363: F, t38365: F, t38367: F, t38371: F, t38375: F, t38377: F, t38382: F, t38384: F, t38387: F, t38389: F, t38391: F, t38393: F, t38395: F, t38398: F, t38404: F, t4041: F, t884: F, t8960: F) -> F {
    let t38406 = F::new(0.25538759935978703638e-4) * t38361 + F::new(0.25538759935978703638e-4) * t38363 - F::new(0.42564599893297839398e-5) * t38365 - F::new(0.85129199786595678796e-5) * t38367 - F::new(0.1064114997332445985e-4) * t38371 - F::new(0.1064114997332445985e-4) * t38375 - F::new(0.53205749866622299248e-5) * t38377 + F::new(0.11974241701863808564e0) * t4041 * t8960 + F::new(0.14635184302277988245e0) * t38382 + F::new(0.59871208509319042821e-1) * t884 * t38384 + F::new(0.85129199786595678796e-5) * t38387 + F::new(0.85129199786595678796e-5) * t38389 - F::new(0.25538759935978703638e-4) * t38391 + F::new(0.25538759935978703638e-4) * t38393 + F::new(0.85129199786595678796e-5) * t38395 + F::new(0.25538759935978703638e-4) * t38398 + F::new(0.12769379967989351819e-4) * t38404;
    t38406
}

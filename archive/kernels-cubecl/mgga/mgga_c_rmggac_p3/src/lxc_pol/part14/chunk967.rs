//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 967/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk967<F: Float>(t2010: F, t2415: F, t4029: F, t27326: F, t35772: F, t40502: F, t40506: F, t40507: F, t40509: F, t40511: F, t40513: F, t40516: F, t40518: F, t40529: F, t40533: F, t40537: F, t40541: F, t40544: F, t5048: F, t5223: F, t665: F, t7703: F, t884: F) -> F {
    let t40547 = t2010 * t2415 * t4029;
    let t40550 = F::cast_from(0.85129199786595678796e-5_f64) * t40502 - t40506 - F::cast_from(0.85129199786595678796e-5_f64) * t40507 - F::cast_from(0.42564599893297839398e-5_f64) * t40509 + F::cast_from(0.25538759935978703638e-4_f64) * t40511 + F::cast_from(0.12769379967989351819e-4_f64) * t40513 - F::cast_from(0.47896966807455234256e0_f64) * t40516 + F::cast_from(0.17961362552795712846e0_f64) * t40518 + F::cast_from(0.11974241701863808564e1_f64) * t5048 * t665 * t5223 + F::cast_from(0.35922725105591425692e0_f64) * t884 * t7703 * t27326 - F::cast_from(0.25538759935978703638e-4_f64) * t40529 - F::cast_from(0.38308139903968055457e-4_f64) * t40533 + F::cast_from(0.51077519871957407276e-4_f64) * t40537 + F::cast_from(0.12769379967989351819e-4_f64) * t40541 - F::cast_from(0.72042316457491791906e-3_f64) * t40544 - F::cast_from(0.36021158228745895953e-3_f64) * t40547 - F::cast_from(0.30487649791575028314e-3_f64) * t35772;
    t40550
}

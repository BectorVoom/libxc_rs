//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 967/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk967(t2010: f64, t2415: f64, t4029: f64, t27326: f64, t35772: f64, t40502: f64, t40506: f64, t40507: f64, t40509: f64, t40511: f64, t40513: f64, t40516: f64, t40518: f64, t40529: f64, t40533: f64, t40537: f64, t40541: f64, t40544: f64, t5048: f64, t5223: f64, t665: f64, t7703: f64, t884: f64) -> f64 {
    let t40547 = t2010 * t2415 * t4029;
    let t40550 = 0.85129199786595678796e-5_f64 * t40502 - t40506 - 0.85129199786595678796e-5_f64 * t40507 - 0.42564599893297839398e-5_f64 * t40509 + 0.25538759935978703638e-4_f64 * t40511 + 0.12769379967989351819e-4_f64 * t40513 - 0.47896966807455234256e0_f64 * t40516 + 0.17961362552795712846e0_f64 * t40518 + 0.11974241701863808564e1_f64 * t5048 * t665 * t5223 + 0.35922725105591425692e0_f64 * t884 * t7703 * t27326 - 0.25538759935978703638e-4_f64 * t40529 - 0.38308139903968055457e-4_f64 * t40533 + 0.51077519871957407276e-4_f64 * t40537 + 0.12769379967989351819e-4_f64 * t40541 - 0.72042316457491791906e-3_f64 * t40544 - 0.36021158228745895953e-3_f64 * t40547 - 0.30487649791575028314e-3_f64 * t35772;
    t40550
}

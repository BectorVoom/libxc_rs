//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 734/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk734(t1959: f64, t607: f64, t1710: f64, t883: f64, t1447: f64, t1912: f64, t1916: f64, t1920: f64, t1444: f64, t1911: f64, t2979: f64, t493: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4717 = 4.0_f64 / 45.0_f64 * t1959 * t607;
    let t4718 = t883 * t1710;
    let t4721 = 4.0_f64 / 135.0_f64 * t1447 * t1912;
    let t4723 = 8.0_f64 / 135.0_f64 * t1447 * t1916;
    let t4725 = 4.0_f64 / 81.0_f64 * t1447 * t1920;
    let t4727 = 2.0_f64 / 45.0_f64 * t1444 * t1912;
    let t4728 = t2979 * t1911;
    let t4730 = 2.0_f64 / 45.0_f64 * t493 * t4728;
    (t4717, t4718, t4721, t4723, t4725, t4727, t4728, t4730)
}

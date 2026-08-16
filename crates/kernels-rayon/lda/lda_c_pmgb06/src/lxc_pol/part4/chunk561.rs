//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 561/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk561(t1067: f64, t2149: f64, t2152: f64, t2154: f64, t2156: f64, t2161: f64, t2395: f64, t2396: f64, t248: f64, t283: f64, t961: f64, t970: f64, t982: f64) -> f64 {
    let t2405 = t248 * t2396 + 0.0197516734986138_f64 * t2395 * t283 - 1.1696447245269292_f64 * t2149 - 8.0_f64 * t2154 - 8.0_f64 * t2156 - 0.0003662289461201309_f64 * t2152 + 2.0_f64 * t2161 - t961 - t970 + t982 + t1067;
    t2405
}

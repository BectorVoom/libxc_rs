//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 533/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk533(t1241: f64, t1249: f64, t1302: f64, t2245: f64, t2694: f64, t2698: f64, t2701: f64, t2704: f64, t2708: f64, t69: f64) -> f64 {
    let t2730 = -t1241 + t2694 + t1249 + t2698 - t2701 + t1302 + 1.1495033333333333_f64 * t2245 + 5.172765_f64 * t69 * t2704 - 1.724255_f64 * t69 * t2708;
    t2730
}

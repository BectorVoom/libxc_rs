//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 441/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk441(t12: f64, t1079: f64, t764: f64, t1: f64, t14: f64, t337: f64, t395: f64, t2132: f64, t257: f64, zeta_threshold: f64) -> (f64, f64) {
    let t13 = t12 <= zeta_threshold;
    let t2133 = t1079 * t764;
    let t2136 = t14 * t1;
    let t2140 = piecewise3(t13, 0.0_f64, 4.0_f64 / 9.0_f64 * t2133 * t337 - 8.0_f64 / 3.0_f64 * t2136 * t395);
    let t2142 = (t2132 + t2140) * t257;
    (t2133, t2142)
}

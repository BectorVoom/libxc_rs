//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1030/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1030(t12396: f64, t13392: f64, t19314: f64, t13388: f64, t13384: f64, t350: f64, t7606: f64, t7613: f64, t337: f64, t7598: f64, t36: f64, t506: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19316 = t12396 * t13392 * t19314;
    let t19319 = t12396 * t13388 * t19314;
    let t19322 = t12396 * t13384 * t19314;
    let t19324 = t350 * t7606;
    let t19326 = t350 * t7613;
    let t19332 = t7598 * t337;
    let t19334 = t36 * t506 * t19332;
    (t19316, t19319, t19322, t19324, t19326, t19332, t19334)
}

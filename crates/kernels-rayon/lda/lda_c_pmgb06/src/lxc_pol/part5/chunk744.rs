//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 744/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk744(t2414: f64, t301: f64, t413: f64, t297: f64, t113: f64, t6716: f64, t2712: f64, t365: f64, t350: f64, t2715: f64, t2696: f64, t348: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6957 = t2414 * t413 * t301;
    let t6958 = t297 * t6957;
    let t6961 = t6716 * t113 * t301;
    let t6967 = t365 * t2712;
    let t6968 = t6967 * t350;
    let t6970 = t365 * t2715;
    let t6971 = t6970 * t350;
    let t6973 = t348 * t2696;
    (t6957, t6958, t6961, t6967, t6968, t6970, t6971, t6973)
}

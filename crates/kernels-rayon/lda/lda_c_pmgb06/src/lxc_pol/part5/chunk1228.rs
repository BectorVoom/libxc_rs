//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1228/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1228(t12657: f64, t12662: f64, t20109: f64, t20111: f64, t20112: f64, t20113: f64, t20115: f64, t20116: f64, t20121: f64, t20122: f64, t20123: f64, t20127: f64) -> f64 {
    let t21955 = t20109 + t20111 + t20112 + t20113 - t20115 + t20116 - 8.0_f64 / 135.0_f64 * t12657 + t12662 + t20121 + t20122 + t20123 - t20127;
    t21955
}

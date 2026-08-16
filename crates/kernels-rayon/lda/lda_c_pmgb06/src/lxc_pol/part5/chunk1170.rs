//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1170/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1170(t1963: f64, t6127: f64, t17875: f64, t14016: f64, t14465: f64, t14467: f64, t14472: f64, t21050: f64, t21052: f64, t21055: f64, t21059: f64, t21061: f64) -> (f64, f64, f64) {
    let t21065 = t6127 * t1963 / 15.0_f64;
    let t21066 = t17875 / 15.0_f64;
    let t21067 = t21050 - t21052 - t21055 - t21059 + 12.0_f64 * t14465 + 4.0_f64 / 3.0_f64 * t21061 + 0.0033101111111111113_f64 * t14467 + t14472 + t21065 - t21066 + t14016;
    (t21065, t21066, t21067)
}

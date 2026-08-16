//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 862/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk862(t7888: f64, t3323: f64, t3326: f64, t3706: f64, t3713: f64, t3715: f64, t3961: f64, t3962: f64, t3963: f64, t7870: f64, t7875: f64, t7879: f64, t7884: f64) -> f64 {
    let t8926 = 24.0_f64 * t7888;
    let t8927 = 0.5476129290375806_f64 * t3323 + 0.5476129290375806_f64 * t3326 + t3706 + 24.0_f64 * t7870 - 24.0_f64 * t7875 + 24.0_f64 * t7879 - 24.0_f64 * t7884 + t8926 + t3961 + t3962 - t3963 + t3713 + t3715;
    t8927
}

//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 861/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk861(t3317: f64, t3319: f64, t3335: f64, t3342: f64, t3692: f64, t3695: f64, t3696: f64, t3697: f64, t3943: f64, t3944: f64, t3945: f64, t7851: f64, t7855: f64) -> f64 {
    let t8918 = 12.0_f64 * t7851 + 12.0_f64 * t7855 - 0.821419393556371_f64 * t3335 - 0.5476129290375806_f64 * t3342 + t3943 + t3944 - t3945 + t3692 + t3695 + t3696 - t3697 + 0.821419393556371_f64 * t3317 + 0.821419393556371_f64 * t3319;
    t8918
}

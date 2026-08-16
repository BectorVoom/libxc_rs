//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 209/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk209(t187: f64, t733: f64, t204: f64, t133: f64, t609: f64, t131: f64, t48: f64, t49: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t734 = t187 * t733;
    let t735 = t734 * t204;
    let t736 = t133 * t609;
    let t737 = t131 * t736;
    let t739 = 2.3693919160612835_f64 * t735 * t737;
    let t741 = 1.0_f64 / t49 / t48;
    (t734, t735, t736, t737, t739, t741)
}

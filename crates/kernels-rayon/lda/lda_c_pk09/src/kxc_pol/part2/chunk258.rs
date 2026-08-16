//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 258/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk258(t1151: f64, t272: f64, t251: f64, t246: f64) -> (f64, f64, f64, f64) {
    let t1153 = 1.28_f64 * t1151 * t272;
    let t1154 = t251 * t251;
    let t1155 = 1.0_f64 / t1154;
    let t1156 = t246 * t1155;
    (t1153, t1154, t1155, t1156)
}

//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 519/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk519(t3104: f64, t898: f64, t905: f64, t747: f64, t838: f64, t923: f64, t748: f64, t909: f64, t913: f64, t3103: f64, t916: f64, t919: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3105 = t3104 * t898;
    let t3107 = t3104 * t905;
    let t3118 = t838 * t747;
    let t3119 = t3118 * t923;
    let t3120 = 29.43979784173208_f64 * t3119;
    let t3121 = t748 * t909;
    let t3123 = t748 * t913;
    let t3129 = t916 * t3103;
    let t3130 = t3129 * t919;
    (t3105, t3107, t3118, t3119, t3120, t3121, t3123, t3130)
}

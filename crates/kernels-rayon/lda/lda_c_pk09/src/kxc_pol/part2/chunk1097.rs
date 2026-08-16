//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1097/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1097(t12058: f64, t12072: f64, t1776: f64, t452: f64, t1971: f64, t2939: f64, t10959: f64, t11066: f64, t11073: f64, t11076: f64, t11529: f64, t11532: f64, t11535: f64, t11539: f64, t11542: f64, t6323: f64, t6337: f64, t6467: f64, t6508: f64, t6550: f64, t6633: f64, t6634: f64, t6638: f64) -> (f64, f64, f64) {
    let t12073 = t12058 + t12072;
    let t12074 = t12073 * t1776;
    let t12075 = t12074 * t452;
    let t12082 = t2939 * t1971;
    let t12099 = 0.505765839233979_f64 * t11066 + 1.011531678467958_f64 * t10959 + 4.0_f64 * t11529 - 4.0_f64 * t11532 - 4.0_f64 * t11535 + 6.0_f64 * t11539 - 4.0_f64 * t11542 + 0.505765839233979_f64 * t11076 + t6633 + 0.168588613077993_f64 * t11073 + t6638 - 0.168588613077993_f64 * t6337 - 0.505765839233979_f64 * t6323 + 1.3333333333333333_f64 * t6550 + t6634 - 1.3333333333333333_f64 * t6508 + 0.168588613077993_f64 * t6467;
    (t12075, t12082, t12099)
}

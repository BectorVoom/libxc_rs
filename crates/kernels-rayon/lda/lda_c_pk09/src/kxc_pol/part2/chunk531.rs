//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 531/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk531(t2971: f64, t809: f64, t3194: f64, t2974: f64, t1062: f64, t975: f64, t721: f64, t943: f64, t150: f64, t119: f64, t805: f64, t1123: f64, t79: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3254 = t809 * t2971;
    let t3255 = t3254 * t3194;
    let t3257 = t3254 * t2974;
    let t3259 = t975 * t1062;
    let t3260 = t3259 * t721;
    let t3262 = t943 * t2971;
    let t3263 = t3262 * t3194;
    let t3265 = t150 * t2971;
    let t3268 = t805 * t119;
    let t3272 = 1.0_f64 / t1123 / t79;
    (t3254, t3255, t3257, t3260, t3262, t3263, t3265, t3268, t3272)
}

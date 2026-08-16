//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1084/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1084(t1672: f64, t2829: f64, t2826: f64, t11128: f64, t462: f64, t2084: f64, t2758: f64, t11352: f64, t1782: f64, t471: f64, t10959: f64, t11066: f64, t11073: f64, t11076: f64, t11529: f64, t11532: f64, t11535: f64, t11539: f64, t11542: f64, t6323: f64, t6337: f64, t6467: f64, t6508: f64, t6550: f64, t7107: f64, t7108: f64, t7112: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11850 = t2829 * t1672;
    let t11852 = t2826 * t1672;
    let t11854 = t462 * t11128;
    let t11857 = t2084 * t2758;
    let t11863 = t11352 * t1782;
    let t11866 = t471 * t11128;
    let t11883 = 0.3056501876701794_f64 * t11066 + 0.6113003753403587_f64 * t10959 + 3.0646056102413666_f64 * t11529 - 3.0646056102413666_f64 * t11532 - 3.0646056102413666_f64 * t11535 + 4.59690841536205_f64 * t11539 - 3.0646056102413666_f64 * t11542 + 0.3056501876701794_f64 * t11076 + t7107 + 0.1018833958900598_f64 * t11073 + t7112 - 0.1018833958900598_f64 * t6337 - 0.3056501876701794_f64 * t6323 + 1.0215352034137888_f64 * t6550 + t7108 - 1.0215352034137888_f64 * t6508 + 0.1018833958900598_f64 * t6467;
    (t11850, t11852, t11854, t11857, t11863, t11866, t11883)
}

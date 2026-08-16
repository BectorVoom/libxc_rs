//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1095/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1095(t11059: f64, t537: f64, t10959: f64, t11066: f64, t11073: f64, t11076: f64, t11529: f64, t11532: f64, t11535: f64, t11539: f64, t11542: f64, t6323: f64, t6337: f64, t6467: f64, t6508: f64, t6550: f64, t7362: f64, t7363: f64, t7367: f64) -> (f64, f64) {
    let t12041 = t537 * t11059;
    let t12058 = 1.2466946262544771_f64 * t11066 + 2.4933892525089543_f64 * t10959 + 12.5_f64 * t11529 - 12.5_f64 * t11532 - 12.5_f64 * t11535 + 18.75_f64 * t11539 - 12.5_f64 * t11542 + 1.2466946262544771_f64 * t11076 + t7362 + 0.41556487541815906_f64 * t11073 + t7367 - 0.41556487541815906_f64 * t6337 - 1.2466946262544771_f64 * t6323 + 4.166666666666667_f64 * t6550 + t7363 - 4.166666666666667_f64 * t6508 + 0.41556487541815906_f64 * t6467;
    (t12041, t12058)
}

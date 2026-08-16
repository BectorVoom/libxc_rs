//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 991/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk991(t5043: f64, t5056: f64, t5158: f64, t5166: f64, t5177: f64, t5193: f64, t6082: f64, t6098: f64, t9623: f64, t9631: f64, t9635: f64, t9742: f64, t9750: f64, t9948: f64, t9952: f64, t9956: f64, t9959: f64) -> f64 {
    let t10657 = 3.2084841915276807_f64 * t9948 + 3.2084841915276807_f64 * t9952 - 3.2084841915276807_f64 * t9956 + 2.1389894610184537_f64 * t9959 - 0.64_f64 * t9623 - 0.21333333333333335_f64 * t9631 - 0.64_f64 * t9635 - 0.64_f64 * t9742 - 0.64_f64 * t9750 - 0.64_f64 * t5043 - 0.21333333333333335_f64 * t5056 + t6082 - 2.1389894610184537_f64 * t5177 + 2.1389894610184537_f64 * t5193 + t6098 - 6.416968383055361_f64 * t5158 + 6.416968383055361_f64 * t5166;
    t10657
}

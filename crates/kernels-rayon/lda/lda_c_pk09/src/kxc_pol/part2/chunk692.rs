//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 692/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk692(t430: f64, t226: f64, t6319: f64, t6325: f64, t433: f64, t5188: f64, t55: f64) -> (f64, f64, f64, f64) {
    let t6529 = t430 * t430;
    let t6530 = 1.0_f64 / t6529;
    let t6531 = t226 * t6530;
    let t6538 = 0.64_f64 * t6319;
    let t6545 = 0.4266666666666667_f64 * t6325;
    let t6547 = t433 * t5188 * t55;
    (t6531, t6538, t6545, t6547)
}

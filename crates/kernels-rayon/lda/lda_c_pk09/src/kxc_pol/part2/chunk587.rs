//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 587/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk587(t1026: f64, t1067: f64, t4165: f64, t87: f64, t3163: f64, t1098: f64, t3498: f64, t609: f64, t650: f64, t96: f64, t839: f64, t106: f64, t4281: f64) -> (f64, f64, f64, f64, f64) {
    let t4451 = t1026 * t1067;
    let t4457 = t87 * t4165;
    let t4459 = t4457 * t3163 / 3.0_f64;
    let t4461 = 2.0_f64 / 9.0_f64 * t1098 * t3498;
    let t4474 = t96 * t650 * t609;
    let t4475 = t839 * t4474;
    let t4478 = 5.0_f64 / 27.0_f64 * t106 * t4281;
    (t4451, t4459, t4461, t4475, t4478)
}

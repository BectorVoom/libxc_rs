//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 83/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk83(t229: f64, t8: f64, t92: f64, t10: f64, t11: f64, t12: f64, t129: f64, t9: f64, t228: f64, t68: f64, t1: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t231 = t229 * t8 * t92;
    let t235 = t10 * t12 * t11;
    let t236 = t9 * t129;
    let t237 = t235 * t236;
    let t240 = t68 * t228 * t11;
    let t242 = 1.0_f64 / t72 / t1;
    (t231, t235, t236, t237, t240, t242)
}

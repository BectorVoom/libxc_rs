//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 51/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk51(t66: f64, t77: f64, t110: f64, t89: f64) -> (f64, f64, f64) {
    let t113 = 2.298454207681025_f64 * t66 + 0.3056501876701794_f64 * t77 + 0.03591334699501599_f64;
    let t114 = t110 * t113;
    let t115 = t114 * t89;
    (t113, t114, t115)
}

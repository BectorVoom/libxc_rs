//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 441/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk441(t1081: f64, t1082: f64, t1086: f64, t1087: f64, t2159: f64, t2163: f64, t2167: f64, t2171: f64, t2175: f64, t2179: f64) -> f64 {
    let t2314 = t1081 + t1082 + 0.6806222787477182_f64 * t2159 + 0.6806222787477182_f64 * t2163 - 0.6806222787477182_f64 * t2167 + t1086 + t1087 + 0.04525483399593904_f64 * t2171 + 0.04525483399593904_f64 * t2175 - 0.04525483399593904_f64 * t2179;
    t2314
}

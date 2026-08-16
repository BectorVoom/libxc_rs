//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 427/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk427(t2159: f64, t2163: f64, t2167: f64, t2171: f64, t2175: f64, t2179: f64, t777: f64, t778: f64, t782: f64, t783: f64, t788: f64, t89: f64) -> (f64, f64, f64) {
    let t2181 = t777 + t778 + 18.75_f64 * t2159 + 18.75_f64 * t2163 - 18.75_f64 * t2167 + t782 + t783 + 1.2466946262544771_f64 * t2171 + 1.2466946262544771_f64 * t2175 - 1.2466946262544771_f64 * t2179;
    let t2182 = t2181 * t788;
    let t2183 = t2182 * t89;
    (t2181, t2182, t2183)
}

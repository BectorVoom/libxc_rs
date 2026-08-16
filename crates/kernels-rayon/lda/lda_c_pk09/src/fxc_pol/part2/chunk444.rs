//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 444/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk444(t2171: f64, t2175: f64, t2179: f64, t966: f64, t967: f64, t974: f64) -> (f64, f64) {
    let t2353 = t966 + t967 + 11.879313099038017_f64 * t2171 + 11.879313099038017_f64 * t2175 - 11.879313099038017_f64 * t2179;
    let t2354 = t2353 * t974;
    (t2353, t2354)
}

//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 244/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk244(t10: f64, t86: f64, t88: f64, t975: f64, t838: f64, t91: f64) -> (f64, f64, f64) {
    let t1058 = t86 * t88 * t10;
    let t1059 = t975 * t1058;
    let t1062 = t838 * t91;
    (t1058, t1059, t1062)
}

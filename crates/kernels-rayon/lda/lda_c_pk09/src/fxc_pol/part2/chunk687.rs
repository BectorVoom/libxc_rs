//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 687/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk687(t6501: f64, t4977: f64, t55: f64, t285: f64, t1751: f64) -> (f64, f64) {
    let t6502 = 3.2084841915276807_f64 * t6501;
    let t6503 = t55 * t4977;
    let t6504 = t285 * t6503;
    let t6505 = t1751 * t6504;
    (t6502, t6505)
}

//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 39/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk39(t66: f64, t77: f64, t83: f64, t34: f64) -> (f64, f64, f64) {
    let t86 = 2.2155652738222966_f64 * t66 + 0.2946275542389858_f64 * t77 + 0.0346182074034769_f64;
    let t87 = t83 * t86;
    let t88 = 1.0_f64 / t34;
    (t86, t87, t88)
}

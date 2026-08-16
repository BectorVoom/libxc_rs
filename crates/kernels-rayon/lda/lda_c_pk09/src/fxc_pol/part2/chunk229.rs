//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 229/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk229(t62: f64, t623: f64, t896: f64, t890: f64, t54: f64, t55: f64) -> (f64, f64, f64, f64) {
    let t897 = t62 * t623;
    let t898 = t896 * t897;
    let t899 = t890 * t898;
    let t902 = 1.0_f64 / t55 / t54;
    (t897, t898, t899, t902)
}

//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 836/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk836(t8128: f64, t917: f64, t8334: f64, t890: f64, t7608: f64, t839: f64, t2254: f64, t4023: f64, t623: f64, t8318: f64, t8322: f64, t8330: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8517 = t917 * t8128;
    let t8519 = t890 * t8334;
    let t8521 = t839 * t7608;
    let t8524 = t4023 * t2254 * t623;
    let t8525 = t890 * t8524;
    let t8527 = t890 * t8318;
    let t8529 = t890 * t8322;
    let t8531 = t917 * t8330;
    (t8517, t8519, t8521, t8524, t8525, t8527, t8529, t8531)
}

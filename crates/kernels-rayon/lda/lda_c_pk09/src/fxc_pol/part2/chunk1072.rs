//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1072/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1072(t2115: f64, t2730: f64, t93: f64, t10962: f64, t11450: f64, t11452: f64, t11456: f64, t11458: f64, t11462: f64, t7224: f64, t7226: f64, t7228: f64, t7230: f64, t7232: f64) -> (f64, f64) {
    let t11633 = t2115 * t2730;
    let t11634 = t93 * t11633;
    let t11657 = -t7224 / 36.0_f64 - t7226 / 18.0_f64 + t7228 / 18.0_f64 + t7230 / 18.0_f64 + t7232 / 18.0_f64 + 0.037002892246025966_f64 * t11450 + 0.037002892246025966_f64 * t11452 - 0.037002892246025966_f64 * t11456 - 0.14975624337724558_f64 * t11458 - 0.14975624337724558_f64 * t11462 + 0.03412591035063918_f64 * t10962;
    (t11634, t11657)
}

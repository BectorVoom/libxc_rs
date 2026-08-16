//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1067/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1067(t10954: f64, t10962: f64, t10966: f64, t11062: f64, t11070: f64, t11556: f64, t11559: f64, t11563: f64, t11566: f64, t11574: f64, t6327: f64, t6519: f64, t6527: f64, t7158: f64, t7165: f64, t7166: f64, t7171: f64) -> f64 {
    let t11577 = 2.9540870317630623_f64 * t6527 - 2.9540870317630623_f64 * t6519 - 1.4770435158815312_f64 * t11556 + 1.4770435158815312_f64 * t11559 - 0.0982091847463286_f64 * t10962 + 0.9846956772543541_f64 * t11563 - 0.9846956772543541_f64 * t11566 - 0.2946275542389858_f64 * t11070 - 0.2946275542389858_f64 * t10954 - 0.2946275542389858_f64 * t10966 - 0.2946275542389858_f64 * t11062 + 1.4770435158815312_f64 * t11574 + 0.2946275542389858_f64 * t6327 + t7158 + t7165 - t7166 - t7171;
    t11577
}

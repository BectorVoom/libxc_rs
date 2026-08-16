//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1308/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1308(t103: f64, t16905: f64, t16910: f64, t17162: f64, t17164: f64, t17166: f64, t17169: f64, t17172: f64, t17175: f64, t17177: f64, t17185: f64, t17190: f64, t17193: f64, t3358: f64, t9967: f64) -> f64 {
    let t17195 = 0.07198333333333333_f64 * t17162 + 0.026660493827160493_f64 * t17164 - 0.3519185185185185_f64 * t17166 - 0.03999074074074074_f64 * t17169 - 0.10664197530864197_f64 * t17172 + 0.14396666666666666_f64 * t17175 + 0.14396666666666666_f64 * t17177 - 0.002962962962962963_f64 * t103 * t3358 * t16905 - 0.006913580246913581_f64 * t103 * t9967 * t16910 - 0.017777777777777778_f64 * t17185 + 0.14396666666666666_f64 * t17190 - 0.21595_f64 * t17193;
    t17195
}

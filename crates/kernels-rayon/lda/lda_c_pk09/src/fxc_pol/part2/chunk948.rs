//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 948/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk948(t9945: f64, t9972: f64, t1435: f64, t2626: f64, t142: f64, t338: f64, t3677: f64, t92: f64, t9946: f64, t3248: f64, t9633: f64, t6037: f64) -> (f64, f64, f64, f64) {
    let t9973 = t9945 + t9972;
    let t9975 = t2626 * t1435;
    let t9977 = t338 * t142;
    let t9978 = t92 * t3677;
    let t9980 = t9977 * t9978 * t9946;
    let t9982 = t3248 * t9633;
    let t9983 = t6037 * t9982;
    (t9973, t9975, t9980, t9983)
}

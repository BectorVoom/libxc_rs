//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1065/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1065(t391: f64, t4435: f64, t1200: f64, t1795: f64, t2813: f64, t868: f64, t107: f64, t410: f64, t4575: f64, t122: f64, t4182: f64, t886: f64) -> (f64, f64, f64, f64, f64) {
    let t11700 = t391 * t4435;
    let t11708 = t1795 * t1200;
    let t11720 = t2813 * t868;
    let t11723 = t107 * t410 * t4575;
    let t11726 = t122 * t4182 * t886;
    (t11700, t11708, t11720, t11723, t11726)
}

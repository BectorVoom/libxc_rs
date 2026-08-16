//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 734/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk734(t176: f64, t6831: f64, t166: f64, t2583: f64, t435: f64, t132: f64, t2563: f64, t490: f64, t1933: f64, t831: f64, t2554: f64, t489: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6832 = t6831 * t176;
    let t6833 = t166 * t6832;
    let t6836 = t435 * t2583;
    let t6837 = t132 * t6836;
    let t6839 = t2563 * t490;
    let t6841 = t831 * t1933;
    let t6843 = t489 * t2554;
    (t6832, t6833, t6836, t6837, t6839, t6841, t6843)
}

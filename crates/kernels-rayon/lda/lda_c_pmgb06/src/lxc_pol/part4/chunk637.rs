//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 637/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk637(t144: f64, t3031: f64, t1600: f64, t511: f64, t1603: f64, t489: f64, t161: f64, t1630: f64, t435: f64, t132: f64, t1547: f64, t478: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3032 = t144 * t3031;
    let t3038 = t511 * t1600;
    let t3043 = t489 * t1603;
    let t3044 = t161 * t3043;
    let t3050 = t435 * t1630;
    let t3051 = t132 * t3050;
    let t3055 = t1547 * t478;
    (t3032, t3038, t3043, t3044, t3050, t3051, t3055)
}

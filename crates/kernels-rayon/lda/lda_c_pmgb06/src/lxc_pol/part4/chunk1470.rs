//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1470/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1470(t26: f64, t2732: f64, t329: f64, t2407: f64, t247: f64, t1156: f64, t123: f64, t2422: f64, t10903: f64, t10905: f64, t1167: f64, t14663: f64, t14666: f64, t14669: f64, t14672: f64, t18437: f64, t305: f64, t6939: f64, t726: f64) -> (f64, f64) {
    let t18939 = t26 * t2732;
    let t18940 = t329 * t18939;
    let t18954 = t247 * t2407;
    let t18969 = t123 * t1156 * t2422;
    let t18973 = -0.2133002709687175_f64 * t14663 + 0.31995040645307626_f64 * t18954 - 0.031835665774679375_f64 * t123 * t305 * t18437 - 0.031835665774679375_f64 * t123 * t1167 * t2422 + 1.0376068845080684_f64 * t14666 + 1.0376068845080684_f64 * t14669 + 0.10611888591559791_f64 * t14672 - 0.06367133154935875_f64 * t123 * t726 * t6939 + 0.10611888591559791_f64 * t18969 + 0.31995040645307626_f64 * t10903 - 2.55960325162461_f64 * t10905;
    (t18940, t18973)
}

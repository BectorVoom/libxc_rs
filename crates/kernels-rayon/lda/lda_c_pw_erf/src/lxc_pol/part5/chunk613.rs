//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 613/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk613(t1085: f64, t4397: f64, t1067: f64, t749: f64, t1070: f64, t1034: f64, t748: f64, t40: f64, t1064: f64, t1077: f64, t1765: f64, t1: f64, t1750: f64, t887: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4398 = t4397 * t1085;
    let t4401 = t1067 * t749;
    let t4403 = t1070 * t749;
    let t4405 = t748 * t1034;
    let t4406 = t40 * t4405;
    let t4408 = t1064 * t749;
    let t4412 = t1765 * t1077;
    let t4415 = t887 * t1750 * t1;
    (t4398, t4401, t4403, t4405, t4406, t4408, t4412, t4415)
}

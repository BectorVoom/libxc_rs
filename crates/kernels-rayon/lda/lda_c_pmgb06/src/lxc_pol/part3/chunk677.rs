//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 677/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk677(t1195: f64, t4194: f64, t115: f64, t1180: f64, t562: f64, t1669: f64, t1194: f64, t113: f64, t247: f64, t395: f64, t2799: f64, t2801: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4196 = 0.02358774_f64 * t4194 * t1195;
    let t4197 = t1180 * t115;
    let t4199 = 0.09753333333333333_f64 * t562 * t4197;
    let t4200 = t1669 * t115;
    let t4202 = 0.03145032_f64 * t1194 * t4200;
    let t4205 = 0.001883059277350998_f64 * t113 * t247 * t115;
    let t4206 = 6.0_f64 * t395;
    let t4207 = 18.0_f64 * t2799;
    let t4208 = 12.0_f64 * t2801;
    (t4196, t4197, t4199, t4200, t4202, t4205, t4206, t4207, t4208)
}

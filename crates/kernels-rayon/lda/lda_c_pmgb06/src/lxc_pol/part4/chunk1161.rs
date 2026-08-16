//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1161/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1161(t11904: f64, t6630: f64, t12497: f64, t5068: f64, t6629: f64, t2088: f64, t764: f64, t337: f64, t5069: f64, t2489: f64, t3198: f64, t1444: f64, t6292: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15270 = 8.0_f64 / 45.0_f64 * t11904 * t6630;
    let t15273 = 8.0_f64 / 45.0_f64 * t5068 * t12497 * t6629;
    let t15274 = t764 * t2088;
    let t15275 = t15274 * t337;
    let t15278 = 8.0_f64 / 45.0_f64 * t5068 * t5069 * t15275;
    let t15280 = 2.0_f64 / 45.0_f64 * t3198 * t2489;
    let t15282 = 4.0_f64 / 45.0_f64 * t1444 * t6292;
    (t15270, t15273, t15274, t15275, t15278, t15280, t15282)
}

//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 751/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk751(t69: f64, t6983: f64, t6986: f64, t2695: f64, t342: f64, t2209: f64, t769: f64, t2448: f64, t2247: f64, t2248: f64, t3505: f64, t3517: f64, t3525: f64, t3644: f64, t5874: f64, t6980: f64, t7017: f64, t7024: f64, t7026: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7069 = t69 * t6983;
    let t7071 = t69 * t6986;
    let t7073 = t2695 * t342;
    let t7077 = t769 * t2209;
    let t7081 = t2448 * t342;
    let t7085 = -t7017 - t3505 - 0.7663355555555555_f64 * t3644 - t3517 + t3525 - 1.724255_f64 * t69 * t6980 - 1.724255_f64 * t7069 + 0.5747516666666667_f64 * t7071 - 20.69106_f64 * t2247 * t5874 * t7073 + 10.34553_f64 * t2247 * t2248 * t7077 + 5.172765_f64 * t2247 * t2248 * t7081 + t7024 - t7026;
    (t7069, t7071, t7073, t7077, t7081, t7085)
}

//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1064/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1064(t2002: f64, t6365: f64, t6275: f64, t6372: f64, t16213: f64, t16215: f64, t16217: f64, t16219: f64, t12252: f64, t132: f64, t137: f64, t2604: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19736 = 2.0_f64 / 15.0_f64 * t2002 * t6365;
    let t19738 = 4.0_f64 / 15.0_f64 * t6275 * t6372;
    let t19739 = 4.0_f64 / 45.0_f64 * t16213;
    let t19740 = 8.0_f64 / 45.0_f64 * t16215;
    let t19741 = 4.0_f64 / 27.0_f64 * t16217;
    let t19742 = 8.0_f64 / 27.0_f64 * t16219;
    let t19746 = t132 * t137 * t12252 * t2604 / 5.0_f64;
    (t19736, t19738, t19739, t19740, t19741, t19742, t19746)
}

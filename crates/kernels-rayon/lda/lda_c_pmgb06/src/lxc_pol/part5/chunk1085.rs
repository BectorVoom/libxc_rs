//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1085/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1085(t1423: f64, t7690: f64, t2477: f64, t5220: f64, t1447: f64, t7685: f64, t10247: f64, t12622: f64, t12633: f64, t1420: f64, t1972: f64, t19870: f64, t2501: f64, t2948: f64, t439: f64, t442: f64, t444: f64, t5187: f64, t6114: f64, t6523: f64, t7524: f64, t7525: f64, t7584: f64, t7585: f64) -> f64 {
    let t20062 = t1423 * t7690;
    let t20064 = t5220 * t2477;
    let t20066 = t1447 * t7685;
    let t20068 = t12622 - 2.0_f64 / 15.0_f64 * t5187 * t2501 + 2.0_f64 / 15.0_f64 * t439 * t12633 * t6523 - t1420 * t7525 / 15.0_f64 - t439 * t2948 * t7524 / 15.0_f64 + t439 * t442 * t444 * t19870 / 45.0_f64 + 8.0_f64 / 81.0_f64 * t1420 * t7585 + 8.0_f64 / 81.0_f64 * t439 * t10247 * t7584 + t1972 * t6114 / 5.0_f64 + 2.0_f64 / 45.0_f64 * t20062 + 4.0_f64 / 45.0_f64 * t20064 + 2.0_f64 / 15.0_f64 * t20066;
    t20068
}

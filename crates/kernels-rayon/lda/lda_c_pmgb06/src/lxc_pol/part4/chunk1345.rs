//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1345/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1345(t17666: f64, t1423: f64, t6255: f64, t1594: f64, t1966: f64, t2604: f64, t439: f64, t9647: f64, t13708: f64, t13710: f64, t17643: f64, t17647: f64, t17650: f64, t17652: f64, t17653: f64, t17655: f64, t17657: f64, t17661: f64, t17662: f64, t17665: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17667 = 8.0_f64 / 45.0_f64 * t17666;
    let t17668 = t1423 * t6255;
    let t17669 = 4.0_f64 / 15.0_f64 * t17668;
    let t17674 = 4.0_f64 / 5.0_f64 * t439 * t1966 * t9647 * t2604 * t1594;
    let t17675 = 8.0_f64 / 405.0_f64 * t13708;
    let t17676 = 8.0_f64 / 135.0_f64 * t13710;
    let t17677 = t17643 - t17647 - t17650 + t17652 - t17653 - t17655 - t17657 + t17661 + t17662 + t17665 + t17667 - t17669 + t17674 - t17675 - t17676;
    (t17667, t17669, t17674, t17675, t17676, t17677)
}

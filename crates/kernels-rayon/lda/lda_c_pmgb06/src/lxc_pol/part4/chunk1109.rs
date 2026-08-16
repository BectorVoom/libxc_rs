//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1109/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1109(t1431: f64, t5220: f64, t1441: f64, t1963: f64, t3220: f64, t1423: f64, t4780: f64, t4615: f64, t1969: f64, t1447: f64, t5337: f64, t5477: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13731 = t5220 * t1431;
    let t13733 = t5220 * t1441;
    let t13740 = t3220 * t1963;
    let t13742 = t1423 * t4780;
    let t13744 = t1423 * t4615;
    let t13748 = t3220 * t1969;
    let t13752 = t1447 * t5337;
    let t13756 = t1447 * t5477;
    (t13731, t13733, t13740, t13742, t13744, t13748, t13752, t13756)
}

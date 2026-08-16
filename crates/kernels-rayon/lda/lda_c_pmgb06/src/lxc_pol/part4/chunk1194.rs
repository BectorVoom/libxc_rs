//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1194/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1194(t10412: f64, t2500: f64, t439: f64, t1423: f64, t6376: f64, t15734: f64, t15736: f64, t15738: f64, t15740: f64, t15743: f64, t15745: f64, t15746: f64, t15747: f64, t15748: f64, t15753: f64, t15754: f64, t15758: f64, t15760: f64) -> (f64, f64, f64) {
    let t15763 = 2.0_f64 / 45.0_f64 * t439 * t10412 * t2500;
    let t15764 = t1423 * t6376;
    let t15765 = 8.0_f64 / 135.0_f64 * t15764;
    let t15766 = t15734 + t15736 - t15738 + t15740 - t15743 - t15745 - t15746 + t15747 - t15748 - t15753 + t15754 - t15758 - t15760 - t15763 + t15765;
    (t15763, t15765, t15766)
}

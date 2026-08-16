//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 966/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk966(t405: f64, t6218: f64, t2639: f64, t955: f64, t2645: f64, t6152: f64, t4913: f64, t6156: f64, t1423: f64, t6551: f64, t6376: f64, t6379: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15654 = t405 * t6218;
    let t15663 = t955 * t2639;
    let t15671 = t955 * t2645;
    let t15675 = t405 * t6152;
    let t15677 = t4913 * t6156;
    let t15739 = t1423 * t6551;
    let t15764 = t1423 * t6376;
    let t15770 = t1423 * t6379;
    (t15654, t15663, t15671, t15675, t15677, t15739, t15764, t15770)
}

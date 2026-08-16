//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 961/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk961(t409: f64, t419: f64, t421: f64, t6716: f64, t1186: f64, t7155: f64, t1447: f64, t6744: f64, t6748: f64, t6791: f64, t1423: f64, t6775: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15163 = t409 * t6716 * t419 * t421;
    let t15166 = t7155 * t1186 * t421;
    let t15180 = t1447 * t6744;
    let t15182 = t1447 * t6748;
    let t15184 = t1447 * t6791;
    let t15189 = t1423 * t6775;
    (t15163, t15166, t15180, t15182, t15184, t15189)
}

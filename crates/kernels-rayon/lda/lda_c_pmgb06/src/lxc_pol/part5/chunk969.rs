//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 969/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk969(t130: f64, t801: f64, t5076: f64, t5082: f64, t830: f64, t5067: f64, t5137: f64, t5499: f64, t6395: f64, t1423: f64, t6361: f64, t6365: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15854 = t801 * t130;
    let t15855 = t15854 * t5076;
    let t15858 = t15854 * t5082;
    let t15861 = t830 * t130;
    let t15862 = t15861 * t5067;
    let t15865 = t15861 * t5137;
    let t15887 = t5499 * t6395;
    let t15891 = t1423 * t6361;
    let t15893 = t1423 * t6365;
    (t15855, t15858, t15862, t15865, t15887, t15891, t15893)
}

//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1201/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1201(t1977: f64, t5194: f64, t1983: f64, t5210: f64, t830: f64, t2462: f64, t3226: f64, t1447: f64, t6533: f64, t131: f64, t4238: f64, t160: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15835 = t5194 * t1977;
    let t15836 = 8.0_f64 / 135.0_f64 * t15835;
    let t15838 = t830 * t5210 * t1983;
    let t15839 = 4.0_f64 / 27.0_f64 * t15838;
    let t15840 = t3226 * t2462;
    let t15841 = 8.0_f64 / 135.0_f64 * t15840;
    let t15842 = t1447 * t6533;
    let t15843 = 8.0_f64 / 135.0_f64 * t15842;
    let t15844 = t131 * t4238;
    let t15845 = t160 * t15844;
    (t15836, t15839, t15841, t15843, t15844, t15845)
}

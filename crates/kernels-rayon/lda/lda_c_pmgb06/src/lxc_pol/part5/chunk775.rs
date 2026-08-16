//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 775/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk775(t5996: f64, t6035: f64, t7106: f64, t7243: f64, t6974: f64, t6977: f64, t2448: f64, t38: f64, t776: f64, t5788: f64, t5803: f64, t2229: f64, t5791: f64, t5797: f64, t5813: f64, t63: f64, t6968: f64, t6971: f64, t6984: f64, t6987: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7245 = t5996 + t6035 + t7106 + t7243;
    let t7261 = 2.923025_f64 * t6974;
    let t7262 = 1.4615125_f64 * t6977;
    let t7270 = 17.53815_f64 * t38 * t776 * t2448;
    let t7271 = 1.9486833333333333_f64 * t5788;
    let t7274 = 0.9743416666666667_f64 * t5803;
    let t7276 = -8.81424_f64 * t6968 + 2.20356_f64 * t6971 - t7261 + t7262 - 3.0_f64 / 2.0_f64 * t6984 + t6987 / 2.0_f64 + 17.62848_f64 * t63 * t2229 * t2448 + t7270 - t7271 - 2.0_f64 / 3.0_f64 * t5791 - 1.46904_f64 * t5797 + t7274 - 2.93808_f64 * t5813;
    (t7245, t7261, t7262, t7270, t7271, t7274, t7276)
}

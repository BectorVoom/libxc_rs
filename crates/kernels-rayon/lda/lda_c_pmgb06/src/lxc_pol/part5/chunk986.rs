//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 986/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk986(t2617: f64, t955: f64, t2620: f64, t405: f64, t6879: f64, t350: f64, t6828: f64, t2546: f64, t947: f64, t2542: f64, t2550: f64, t6885: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17030 = t955 * t2617;
    let t17035 = t955 * t2620;
    let t17041 = t405 * t6879;
    let t17054 = t350 * t6828;
    let t17059 = t947 * t2546;
    let t17061 = t947 * t2542;
    let t17066 = t947 * t2550;
    let t17127 = t405 * t6885;
    (t17030, t17035, t17041, t17054, t17059, t17061, t17066, t17127)
}

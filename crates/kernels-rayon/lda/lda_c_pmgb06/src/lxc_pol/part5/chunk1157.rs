//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1157/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1157(t10335: f64, t17771: f64, t20888: f64, t20890: f64, t20892: f64, t20894: f64, t20895: f64, t20896: f64, t20897: f64, t20898: f64, t20901: f64, t1385: f64, t439: f64, t6217: f64, t822: f64) -> (f64, f64, f64, f64) {
    let t20902 = 8.0_f64 / 1215.0_f64 * t10335;
    let t20903 = t17771 / 15.0_f64;
    let t20904 = -t20888 - t20890 - t20892 + t20894 + t20895 + t20896 - t20897 + t20898 + t20901 + t20902 + t20903;
    let t20914 = t439 * t1385 * t6217 * t822 / 15.0_f64;
    (t20902, t20903, t20904, t20914)
}

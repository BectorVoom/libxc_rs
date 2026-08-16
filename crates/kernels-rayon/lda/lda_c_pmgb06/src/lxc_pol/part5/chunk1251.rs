//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1251/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1251(t10350: f64, t10353: f64, t10356: f64, t10358: f64, t10362: f64, t18329: f64, t18331: f64, t20902: f64, t20903: f64, t20914: f64, t20919: f64, t20920: f64, t20922: f64, t20925: f64, t20929: f64, t20931: f64, t20934: f64, t20937: f64, t20940: f64, t20943: f64, t20946: f64, t20950: f64, t20953: f64) -> (f64, f64) {
    let t22036 = t20902 + t20903 - 2.0_f64 / 9.0_f64 * t10350 - 0.013506172839506173_f64 * t10353 - t10356 - t10358 + t10362 - 2.0_f64 / 15.0_f64 * t18329 + 2.0_f64 / 45.0_f64 * t18331 - t20914 - t20919;
    let t22037 = t20920 - t20922 - t20925 - t20929 - t20931 - t20934 - t20937 + t20940 - t20943 - t20946 - t20950 - t20953;
    (t22036, t22037)
}

//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1042/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1042(t12154: f64, t19471: f64, t439: f64, t1: f64, t6150: f64, t2010: f64, t5260: f64, t15764: f64, t15770: f64, t15772: f64, t15774: f64, t19458: f64, t19461: f64, t19463: f64, t19466: f64, t19469: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19474 = 88.0_f64 / 243.0_f64 * t439 * t12154 * t19471;
    let t19475 = t6150 * t1;
    let t19478 = 16.0_f64 / 27.0_f64 * t2010 * t5260 * t19475;
    let t19479 = 4.0_f64 / 45.0_f64 * t15764;
    let t19480 = 4.0_f64 / 15.0_f64 * t15770;
    let t19481 = 4.0_f64 / 9.0_f64 * t15772;
    let t19482 = 16.0_f64 / 45.0_f64 * t15774;
    let t19483 = -t19458 + t19461 + t19463 + t19466 + t19469 + t19474 + t19478 + t19479 + t19480 - t19481 - t19482;
    (t19474, t19475, t19478, t19479, t19480, t19481, t19482, t19483)
}

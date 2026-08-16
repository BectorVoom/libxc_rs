//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1166/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1166(t5211: f64, t5248: f64, t5264: f64, t2952: f64, t439: f64, t5482: f64, t2970: f64, t6494: f64, t3104: f64, t6498: f64, t4619: f64, t464: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13920 = t5211 * t5248;
    let t13921 = 4.0_f64 / 9.0_f64 * t13920;
    let t13922 = t5211 * t5264;
    let t13923 = 10.0_f64 / 27.0_f64 * t13922;
    let t13926 = t439 * t5482 * t2952 / 15.0_f64;
    let t13929 = 2.0_f64 / 15.0_f64 * t439 * t6494 * t2970;
    let t13932 = t439 * t6498 * t3104 / 9.0_f64;
    let t13933 = t4619 * t464;
    (t13921, t13923, t13926, t13929, t13932, t13933)
}

//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1096/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1096(t2010: f64, t6155: f64, t6498: f64, t1385: f64, t439: f64, t477: f64, t7497: f64, t1897: f64, t19802: f64, t1972: f64, t6791: f64, t16513: f64, t1907: f64) -> (f64, f64, f64, f64, f64) {
    let t20182 = 4.0_f64 / 9.0_f64 * t2010 * t6498 * t6155;
    let t20186 = t439 * t1385 * t7497 * t477 / 45.0_f64;
    let t20189 = 2.0_f64 / 45.0_f64 * t439 * t1897 * t19802;
    let t20191 = 2.0_f64 / 15.0_f64 * t1972 * t6791;
    let t20194 = t439 * t16513 * t1907 / 15.0_f64;
    (t20182, t20186, t20189, t20191, t20194)
}

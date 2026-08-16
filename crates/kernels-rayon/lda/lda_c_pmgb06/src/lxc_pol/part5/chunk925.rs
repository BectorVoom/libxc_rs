//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 925/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk925(t1992: f64, t3457: f64, t517: f64, t5312: f64, t1710: f64, t830: f64, t500: f64, t2010: f64, t806: f64, t1435: f64, t1872: f64, t132: f64, t1547: f64, t2107: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12006 = t1992 * t3457;
    let t12012 = t5312 * t517;
    let t12036 = t830 * t1710;
    let t12037 = t12036 * t500;
    let t12038 = 2.0_f64 / 135.0_f64 * t12037;
    let t12041 = t2010 * t806;
    let t12092 = t1435 * t1872;
    let t12112 = t132 * t1547 * t2107;
    (t12006, t12012, t12036, t12038, t12041, t12092, t12112)
}

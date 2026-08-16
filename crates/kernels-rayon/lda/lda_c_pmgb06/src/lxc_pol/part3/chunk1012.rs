//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1012/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1012(t1420: f64, t5350: f64, t1710: f64, t830: f64, t500: f64, t1417: f64, t5194: f64, t2010: f64, t806: f64, t497: f64, t517: f64, t1981: f64, t496: f64, t529: f64) -> (f64, f64, f64, f64, f64) {
    let t12035 = 2.0_f64 / 5.0_f64 * t1420 * t5350;
    let t12036 = t830 * t1710;
    let t12037 = t12036 * t500;
    let t12038 = 2.0_f64 / 135.0_f64 * t12037;
    let t12039 = t5194 * t1417;
    let t12040 = 4.0_f64 / 45.0_f64 * t12039;
    let t12041 = t2010 * t806;
    let t12042 = 8.0_f64 / 1215.0_f64 * t12041;
    let t12043 = t517 * t497;
    let t12047 = 2.0_f64 / 15.0_f64 * t1981 * t496 * t12043 * t529;
    (t12035, t12038, t12040, t12042, t12047)
}

//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 933/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk933(t12661: f64, t1414: f64, t1601: f64, t1908: f64, t3213: f64, t464: f64, t4779: f64, t4103: f64, t872: f64, t486: f64, t5044: f64, t1554: f64, t161: f64, t1836: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12662 = 2.0_f64 / 45.0_f64 * t12661;
    let t12691 = t1601 * t1414;
    let t12752 = t3213 * t1908;
    let t12753 = 2.0_f64 / 135.0_f64 * t12752;
    let t12772 = t4779 * t464;
    let t12804 = t872 * t4103;
    let t12828 = t486 * t5044;
    let t12829 = t12828 / 45.0_f64;
    let t12831 = t161 * t1554 * t1836;
    (t12662, t12691, t12753, t12772, t12804, t12829, t12831)
}

//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1090/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1090(t9828: f64, t9830: f64, t9832: f64, t9834: f64, t9837: f64, t9847: f64, t9853: f64, t12514: f64, t1461: f64, t5065: f64, t5140: f64, t2987: f64, t5068: f64, t5090: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12973 = 4.0_f64 / 135.0_f64 * t9828;
    let t12974 = 2.0_f64 / 45.0_f64 * t9830;
    let t12975 = 2.0_f64 / 45.0_f64 * t9832;
    let t12976 = 4.0_f64 / 45.0_f64 * t9834;
    let t12977 = 2.0_f64 / 45.0_f64 * t9837;
    let t12978 = 4.0_f64 / 45.0_f64 * t9847;
    let t12979 = 4.0_f64 / 45.0_f64 * t9853;
    let t12981 = t5065 * t12514 * t1461;
    let t12982 = t12981 * t5140;
    let t12983 = 4.0_f64 / 27.0_f64 * t12982;
    let t12986 = 2.0_f64 / 15.0_f64 * t5068 * t5090 * t2987;
    (t12973, t12974, t12975, t12976, t12977, t12978, t12979, t12983, t12986)
}

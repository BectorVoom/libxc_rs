//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1331/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1331(t5686: f64, t5688: f64, t11332: f64, t11350: f64, t11367: f64, t11384: f64, t11467: f64, t15290: f64, t15295: f64, t15298: f64, t2951: f64, t2981: f64, t4289: f64, t4292: f64, t4410: f64, t5945: f64, t7: f64, t7333: f64, t7334: f64, t7335: f64, t8106: f64, t8107: f64, t8108: f64, t8109: f64, t8110: f64, t8113: f64, t8114: f64) -> f64 {
    let t15306 = 6.0_f64 * t5686;
    let t15307 = 24.0_f64 * t5688;
    let t15308 = 0.05925536910769562_f64 * t4410 + t7333 - t7334 + t7335 - t8106 + t8107 - t8108 + t8109 + t7 * (t11332 + t11350 + t11367 + t11384 + t11467 + t15290 + t15295 + t15298) - t8110 - t4289 - t8113 + t8114 + 10.526802115419367_f64 * t2951 - 5.694518669548362_f64 * t4292 + t2981 - 3.0_f64 * t5945 + t15306 - t15307;
    t15308
}

//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1051/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1051(t12311: f64, t108: f64, t2075: f64, t267: f64, t3979: f64, t1278: f64, t4488: f64, t4495: f64, t6710: f64, t1390: f64, t1458: f64, t1245: f64) -> (f64, f64, f64, f64, f64) {
    let t12312 = 8.0_f64 / 15.0_f64 * t12311;
    let t12314 = t2075 * t108 * t267;
    let t12316 = 16.0_f64 / 15.0_f64 * t12314 * t3979;
    let t12320 = 8.0_f64 / 15.0_f64 * t4488 * t6710 * t4495 * t1278;
    let t12321 = t1458 * t1390;
    let t12322 = t12321 * t1245;
    (t12312, t12316, t12320, t12321, t12322)
}

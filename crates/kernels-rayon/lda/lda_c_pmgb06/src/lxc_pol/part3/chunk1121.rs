//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1121/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1121(t1531: f64, t1593: f64, t12521: f64, t5077: f64, t13007: f64, t5091: f64, t12555: f64, t5095: f64, t13002: f64, t5084: f64, t1386: f64, t3290: f64, t822: f64) -> (f64, f64, f64, f64, f64) {
    let t13308 = t1593 * t1531;
    let t13311 = 4.0_f64 / 15.0_f64 * t5077 * t13308 * t12521;
    let t13312 = t13007 * t5091;
    let t13313 = 8.0_f64 / 45.0_f64 * t13312;
    let t13314 = t12555 * t5095;
    let t13315 = 8.0_f64 / 45.0_f64 * t13314;
    let t13318 = 2.0_f64 / 5.0_f64 * t5077 * t5084 * t13002;
    let t13322 = 4.0_f64 / 15.0_f64 * t5077 * t3290 * t822 * t1386;
    (t13311, t13313, t13315, t13318, t13322)
}

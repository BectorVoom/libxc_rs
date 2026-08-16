//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1096/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1096(t1464: f64, t1639: f64, t5071: f64, t5138: f64, t2865: f64, t3032: f64, t5077: f64, t822: f64, t2965: f64, t5078: f64, t1601: f64, t12693: f64) -> (f64, f64, f64, f64) {
    let t13053 = t1639 * t1464;
    let t13056 = 2.0_f64 / 9.0_f64 * t5138 * t13053 * t5071;
    let t13060 = 2.0_f64 / 5.0_f64 * t5077 * t3032 * t822 * t2865;
    let t13063 = 4.0_f64 / 15.0_f64 * t5077 * t5078 * t2965;
    let t13064 = t1601 * t1464;
    let t13067 = 2.0_f64 / 9.0_f64 * t5138 * t13064 * t12693;
    (t13056, t13060, t13063, t13067)
}

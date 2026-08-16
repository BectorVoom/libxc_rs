//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1115/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1115(t20391: f64, t5068: f64, t5069: f64, t12546: f64, t19314: f64, t13304: f64, t5138: f64, t5139: f64, t12529: f64, t13300: f64, t5077: f64, t5078: f64, t6364: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20394 = 2.0_f64 / 15.0_f64 * t5068 * t5069 * t20391;
    let t20397 = 2.0_f64 / 5.0_f64 * t5068 * t12546 * t19314;
    let t20400 = 2.0_f64 / 3.0_f64 * t5138 * t13304 * t19314;
    let t20403 = t5138 * t5139 * t20391 / 9.0_f64;
    let t20406 = 8.0_f64 / 27.0_f64 * t12529 * t13300 * t19314;
    let t20409 = 4.0_f64 / 15.0_f64 * t5077 * t5078 * t6364;
    (t20394, t20397, t20400, t20403, t20406, t20409)
}

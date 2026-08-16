//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 913/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk913(t1447: f64, t2980: f64, t1423: f64, t2949: f64, t2866: f64, t1426: f64, t1592: f64, t3238: f64, t517: f64, t1427: f64, t3213: f64, t1710: f64, t431: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10269 = t1447 * t2980;
    let t10273 = t1423 * t2949;
    let t10286 = t1423 * t2866;
    let t10288 = t1426 * t1592;
    let t10293 = t3238 * t517;
    let t10316 = t3213 * t1427;
    let t10318 = t431 * t1710;
    (t10269, t10273, t10286, t10288, t10293, t10316, t10318)
}

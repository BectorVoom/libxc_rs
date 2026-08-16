//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 903/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk903(t132: f64, t3291: f64, t435: f64, t3295: f64, t432: f64, t1447: f64, t3169: f64, t1423: f64, t3186: f64, t1560: f64, t3220: f64, t3213: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9777 = t132 * t435 * t3291;
    let t9781 = t432 * t3295;
    let t9805 = t1447 * t3169;
    let t9821 = t1423 * t3186;
    let t9826 = t3220 * t1560;
    let t9828 = t3213 * t1560;
    (t9777, t9781, t9805, t9821, t9826, t9828)
}

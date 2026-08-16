//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 640/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk640(t446: f64, t5220: f64, t1426: f64, t153: f64, t3279: f64, t3260: f64, t2042: f64, t435: f64, t132: f64, t1847: f64, t224: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5222 = 4.0_f64 / 135.0_f64 * t5220 * t446;
    let t5225 = t1426 * t153;
    let t5253 = t3279 * t153;
    let t5260 = t3260 * t153;
    let t5302 = t435 * t2042;
    let t5304 = 2.0_f64 / 45.0_f64 * t132 * t5302;
    let t5305 = t1847 * t224;
    (t5222, t5225, t5253, t5260, t5302, t5304, t5305)
}

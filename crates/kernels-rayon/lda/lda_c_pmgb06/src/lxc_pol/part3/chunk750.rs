//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 750/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk750(t1386: f64, t5078: f64, t5077: f64, t1435: f64, t5066: f64, t5075: f64) -> (f64, f64, f64, f64) {
    let t5079 = t5078 * t1386;
    let t5081 = 4.0_f64 / 45.0_f64 * t5077 * t5079;
    let t5082 = t5066 * t1435;
    let t5083 = t5075 * t5082;
    (t5079, t5081, t5082, t5083)
}

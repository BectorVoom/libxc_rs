//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 633/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk633(t2107: f64, t435: f64, t132: f64, t1592: f64, t813: f64, t1392: f64, t802: f64, t1461: f64, t5066: f64, t5065: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5115 = t435 * t2107;
    let t5117 = 2.0_f64 / 45.0_f64 * t132 * t5115;
    let t5118 = t813 * t1592;
    let t5126 = 2.0_f64 / 45.0_f64 * t802 * t1392;
    let t5137 = t5066 * t1461;
    let t5138 = t5065 * t5137;
    (t5115, t5117, t5118, t5126, t5137, t5138)
}

//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1205/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1205(t1349: f64, t1953: f64, t20911: f64, t11: f64, t21207: f64, t21211: f64, t352: f64, t7365: f64, t9410: f64, t10102: f64, t34: f64, t6383: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21802 = t1953 * t1349 * t20911;
    let t21805 = t11 * t1349 * t21207;
    let t21808 = t1953 * t1349 * t21211;
    let t21811 = t9410 * t7365 * t352;
    let t21813 = t11 * t10102 * t21811;
    let t21815 = t6383 * t34;
    (t21802, t21805, t21808, t21811, t21813, t21815)
}

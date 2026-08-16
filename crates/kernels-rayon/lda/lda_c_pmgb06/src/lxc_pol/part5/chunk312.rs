//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 312/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk312(t1121: f64, t1122: f64, t123: f64, t317: f64, t701: f64, t740: f64, t247: f64, t83: f64) -> (f64, f64, f64) {
    let t1124 = 0.01084358130030174_f64 * t1121 * t1122;
    let t1133 = t123 * t740 * t701 * t317;
    let t1135 = t247 * t83;
    (t1124, t1133, t1135)
}

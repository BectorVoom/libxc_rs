//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 342/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk342(t1186: f64, t29: f64, t563: f64, t115: f64, t410: f64, t562: f64, t113: f64, t97: f64) -> (f64, f64, f64, f64, f64) {
    let t1187 = t1186 * t29;
    let t1189 = 0.0627_f64 * t1187 * t563;
    let t1190 = t410 * t115;
    let t1192 = 0.0418_f64 * t562 * t1190;
    let t1193 = t113 * t97;
    (t1187, t1189, t1190, t1192, t1193)
}

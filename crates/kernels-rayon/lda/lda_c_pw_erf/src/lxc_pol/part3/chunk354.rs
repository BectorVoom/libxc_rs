//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 354/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk354(t1278: f64, t530: f64, t186: f64, t185: f64, t202: f64, t563: f64, t184: f64) -> (f64, f64, f64, f64, f64) {
    let t1279 = t530 * t1278;
    let t1280 = t186 * t1279;
    let t1282 = 2.0_f64 / 15.0_f64 * t185 * t1280;
    let t1283 = t202 * t563;
    let t1284 = t1283 * t184;
    (t1279, t1280, t1282, t1283, t1284)
}

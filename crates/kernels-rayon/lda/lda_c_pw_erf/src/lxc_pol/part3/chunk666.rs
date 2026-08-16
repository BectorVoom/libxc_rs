//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 666/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk666(t203: f64, t4024: f64, t184: f64, t221: f64, t1519: f64, t511: f64, t1508: f64, t515: f64, t1513: f64, t1522: f64, t563: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4025 = t203 * t4024;
    let t4026 = t4025 * t184;
    let t4028 = 2.0_f64 / 15.0_f64 * t4026 * t221;
    let t4029 = t511 * t1519;
    let t4030 = 4.0_f64 / 45.0_f64 * t4029;
    let t4031 = t1508 * t515;
    let t4032 = 4.0_f64 / 15.0_f64 * t4031;
    let t4033 = t1513 * t515;
    let t4034 = 8.0_f64 / 15.0_f64 * t4033;
    let t4035 = t1522 * t563;
    let t4036 = t4035 * t184;
    let t4038 = 4.0_f64 / 5.0_f64 * t4036 * t221;
    (t4025, t4026, t4028, t4029, t4030, t4031, t4032, t4033, t4034, t4035, t4036, t4038)
}

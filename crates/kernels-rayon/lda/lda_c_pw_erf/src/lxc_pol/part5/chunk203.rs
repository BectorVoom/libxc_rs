//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 203/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk203(t352: f64, t558: f64, t557: f64, t11: f64, t556: f64, t203: f64, t184: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t559 = t558 * t352;
    let t560 = t557 * t559;
    let t561 = t11 * t560;
    let t563 = t556 + 0.0018891666666666666_f64 * t561;
    let t564 = t203 * t563;
    let t565 = t564 * t184;
    (t559, t560, t561, t563, t564, t565)
}

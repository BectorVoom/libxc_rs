//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1040/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1040(t3416: f64, t6894: f64, t1519: f64, t2407: f64, t184: f64, t563: f64, t811: f64, t3899: f64, t571: f64, t6194: f64, t4738: f64, t4946: f64) -> (f64, f64, f64, f64, f64) {
    let t18523 = t3416 * t6894;
    let t18551 = t2407 * t1519;
    let t18555 = t811 * t563 * t184;
    let t18575 = t571 * t3899 * t6194;
    let t18584 = t4738 * t4946;
    (t18523, t18551, t18555, t18575, t18584)
}

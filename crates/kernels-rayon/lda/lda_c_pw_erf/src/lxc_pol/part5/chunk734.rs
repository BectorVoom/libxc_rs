//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 734/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk734(t551: f64, t6601: f64, t2473: f64, t511: f64, t4592: f64, t2400: f64, t563: f64, t184: f64, t221: f64, t3530: f64, t3997: f64, t4600: f64, t4602: f64, t4605: f64, t4607: f64, t6502: f64, t6505: f64, t6508: f64, t6533: f64, t6536: f64, t6539: f64, t6542: f64, t6545: f64, t6547: f64, t6549: f64, t6562: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6603 = 4.0_f64 / 15.0_f64 * t6601 * t551;
    let t6605 = 4.0_f64 / 15.0_f64 * t511 * t2473;
    let t6606 = 8.0_f64 / 135.0_f64 * t4592;
    let t6610 = t2400 * t563;
    let t6611 = t6610 * t184;
    let t6613 = 4.0_f64 / 15.0_f64 * t6611 * t221;
    let t6629 = t3997 + 0.0008396296296296296_f64 * t3530 + 0.0016792592592592592_f64 * t4600 - 0.0008396296296296296_f64 * t4602 + t4605 - 0.002518888888888889_f64 * t4607 - 0.0004198148148148148_f64 * t6549 + 0.002099074074074074_f64 * t6505 - 0.007556666666666666_f64 * t6502 + 0.005037777777777778_f64 * t6508 + 0.0012594444444444445_f64 * t6545 + 0.011335_f64 * t6533 - 0.015113333333333333_f64 * t6536 - 0.0006297222222222223_f64 * t6547 + 0.0012594444444444445_f64 * t6542 - 0.003778333333333333_f64 * t6539 + 0.0018891666666666666_f64 * t6562;
    (t6603, t6605, t6606, t6610, t6611, t6613, t6629)
}

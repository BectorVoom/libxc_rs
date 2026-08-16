//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 734/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk734<F: Float>(t551: F, t6601: F, t2473: F, t511: F, t4592: F, t2400: F, t563: F, t184: F, t221: F, t3530: F, t3997: F, t4600: F, t4602: F, t4605: F, t4607: F, t6502: F, t6505: F, t6508: F, t6533: F, t6536: F, t6539: F, t6542: F, t6545: F, t6547: F, t6549: F, t6562: F) -> (F, F, F, F, F, F, F) {
    let t6603 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t6601 * t551;
    let t6605 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t511 * t2473;
    let t6606 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t4592;
    let t6610 = t2400 * t563;
    let t6611 = t6610 * t184;
    let t6613 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t6611 * t221;
    let t6629 = t3997 + F::cast_from(0.0008396296296296296_f64) * t3530 + F::cast_from(0.0016792592592592592_f64) * t4600 - F::cast_from(0.0008396296296296296_f64) * t4602 + t4605 - F::cast_from(0.002518888888888889_f64) * t4607 - F::cast_from(0.0004198148148148148_f64) * t6549 + F::cast_from(0.002099074074074074_f64) * t6505 - F::cast_from(0.007556666666666666_f64) * t6502 + F::cast_from(0.005037777777777778_f64) * t6508 + F::cast_from(0.0012594444444444445_f64) * t6545 + F::cast_from(0.011335_f64) * t6533 - F::cast_from(0.015113333333333333_f64) * t6536 - F::cast_from(0.0006297222222222223_f64) * t6547 + F::cast_from(0.0012594444444444445_f64) * t6542 - F::cast_from(0.003778333333333333_f64) * t6539 + F::cast_from(0.0018891666666666666_f64) * t6562;
    (t6603, t6605, t6606, t6610, t6611, t6613, t6629)
}

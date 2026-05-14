//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 857/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk857<F: Float>(t172: F, t2441: F, t184: F, t496: F, t202: F, t2423: F, t551: F, t2473: F, t511: F, t4592: F, t2400: F, t563: F, t221: F, t3530: F, t3997: F, t4600: F, t4602: F, t4605: F, t4607: F, t6502: F, t6505: F, t6508: F, t6533: F, t6536: F, t6539: F, t6542: F, t6545: F, t6547: F, t6549: F, t6562: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6596 = t172 * t2441;
    let t6597 = t6596 * t184;
    let t6599 = 4.0 / 15.0 * t6597 * t496;
    let t6600 = t202 * t2423;
    let t6601 = t6600 * t184;
    let t6603 = 4.0 / 15.0 * t6601 * t551;
    let t6605 = 4.0 / 15.0 * t511 * t2473;
    let t6606 = 8.0 / 135.0 * t4592;
    let t6610 = t2400 * t563;
    let t6611 = t6610 * t184;
    let t6613 = 4.0 / 15.0 * t6611 * t221;
    let t6629 = t3997 + 0.0008396296296296296 * t3530 + 0.0016792592592592592 * t4600 - 0.0008396296296296296 * t4602 + t4605 - 0.002518888888888889 * t4607 - 0.0004198148148148148 * t6549 + 0.002099074074074074 * t6505 - 0.007556666666666666 * t6502 + 0.005037777777777778 * t6508 + 0.0012594444444444445 * t6545 + 0.011335 * t6533 - 0.015113333333333333 * t6536 - 0.0006297222222222223 * t6547 + 0.0012594444444444445 * t6542 - 0.003778333333333333 * t6539 + 0.0018891666666666666 * t6562;
    (t6596, t6597, t6599, t6600, t6601, t6603, t6605, t6606, t6610, t6611, t6613, t6629)
}

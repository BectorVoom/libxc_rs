//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1356/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1356<F: Float>(t14667: F, t14683: F, t14685: F, t14688: F, t14691: F, t102: F, t120: F, t19097: F, t128: F, t18826: F, t19653: F, t5651: F, t14718: F, t14777: F, t14843: F, t14846: F, t14849: F, t14851: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19678 = 5.84605 * t14667;
    let t19679 = 2.5982444444444446 * t14683;
    let t19680 = 2.5982444444444446 * t14685;
    let t19681 = 0.9743416666666667 * t14688;
    let t19682 = 1.2991222222222223 * t14691;
    let t19685 = 2.923025 * t102 * t120 * t19097;
    let t19688 = 11.6921 * t102 * t128 * t18826;
    let t19693 = t5651 * t19653;
    let t19696 = 3.031285185185185 * t14718;
    let t19697 = t19678 - t19679 - t19680 + t19681 + t19682 - t19685 + t19688 - 8.0 / 9.0 * t14843 + 1.46904 * t14846 - 1.95872 * t14849 + t14851 / 3.0 - 24.0 * t14777 * t19693 + t19696;
    (t19678, t19679, t19680, t19681, t19682, t19685, t19688, t19693, t19696, t19697)
}

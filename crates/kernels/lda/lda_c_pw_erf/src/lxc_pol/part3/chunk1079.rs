//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1079/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1079<F: Float>(t14691: F, t102: F, t120: F, t14632: F, t1870: F, t1872: F, t436: F, t473: F, t5639: F, t5643: F, t5647: F, t1814: F, t1953: F, t14674: F, t14681: F, t14684: F, t14686: F, t14689: F, t1568: F, t1832: F, t1871: F, t3222: F, t3251: F, t411: F, t5548: F, t756: F) -> (F, F, F, F) {
    let t14692 = 0.9743416666666667 * t14691;
    let t14695 = 2.923025 * t102 * t120 * t14632;
    let t14698 = t1870 * t473 * t436 * t1872;
    let t14701 = t1870 * t5639 * t5643;
    let t14704 = t1870 * t5639 * t5647;
    let t14718 = t1814 * t1953;
    let t14719 = 1.5156425925925925 * t14718;
    let t14720 = 103.4553 * t1870 * t14674 * t756 * t3222 + 20.69106 * t14681 - t14684 - t14686 + t14689 + t14692 - t14695 + 6.89702 * t14698 - 10.34553 * t14701 - 5.172765 * t14704 + 15.518295 * t1870 * t1871 * t5548 * t411 + 15.518295 * t1870 * t1871 * t1832 * t1568 + 5.172765 * t1870 * t1871 * t756 * t3251 + t14719;
    (t14692, t14695, t14719, t14720)
}

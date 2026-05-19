//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1242/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1242<F: Float>(t1870: F, t5639: F, t5647: F, t1814: F, t1953: F, t14674: F, t14681: F, t14684: F, t14686: F, t14689: F, t14692: F, t14695: F, t14698: F, t14701: F, t1568: F, t1832: F, t1871: F, t3222: F, t3251: F, t411: F, t5548: F, t756: F) -> (F, F) {
    let t14704 = t1870 * t5639 * t5647;
    let t14718 = t1814 * t1953;
    let t14719 = F::cast_from(1.5156425925925925_f64) * t14718;
    let t14720 = F::new(103.4553) * t1870 * t14674 * t756 * t3222 + F::new(20.69106) * t14681 - t14684 - t14686 + t14689 + t14692 - t14695 + F::new(6.89702) * t14698 - F::new(10.34553) * t14701 - F::new(5.172765) * t14704 + F::new(15.518295) * t1870 * t1871 * t5548 * t411 + F::new(15.518295) * t1870 * t1871 * t1832 * t1568 + F::new(5.172765) * t1870 * t1871 * t756 * t3251 + t14719;
    (t14719, t14720)
}

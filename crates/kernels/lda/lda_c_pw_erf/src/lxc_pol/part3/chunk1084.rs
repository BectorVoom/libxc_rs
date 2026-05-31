//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1084/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1084<F: Float>(t1289: F, t5211: F, t2076: F, t3565: F, t3660: F, t1325: F, t4632: F, t4829: F, t940: F, t1997: F, t3745: F, t3859: F, t5413: F) -> (F, F, F, F, F, F) {
    let t12681 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t5211 * t1289;
    let t12683 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t2076 * t3565;
    let t12684 = t2076 * t3660;
    let t12685 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t12684;
    let t12689 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t1325 * t4829 * t4632 * t940;
    let t12691 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t3745 * t1997;
    let t12693 = t1325 * t3859 * t5413;
    (t12681, t12683, t12685, t12689, t12691, t12693)
}

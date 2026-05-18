//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1082/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1082<F: Float>(t2171: F, t3880: F, t3884: F, t9602: F, t1383: F, t1960: F, t3657: F, t822: F, t9619: F, t1289: F, t6851: F, t9621: F) -> (F, F, F, F, F, F, F, F) {
    let t12665 = t2171 * t3880;
    let t12666 = F::new(8.0) / F::new(45.0) * t12665;
    let t12667 = t2171 * t3884;
    let t12668 = F::new(8.0) / F::new(27.0) * t12667;
    let t12669 = F::new(8.0) / F::new(15.0) * t9602;
    let t12671 = F::new(2.0) / F::new(5.0) * t1960 * t1383;
    let t12673 = F::new(2.0) / F::new(15.0) * t822 * t3657;
    let t12674 = F::new(8.0) / F::new(45.0) * t9619;
    let t12676 = F::new(4.0) / F::new(5.0) * t6851 * t1289;
    let t12677 = F::new(8.0) / F::new(15.0) * t9621;
    (t12666, t12668, t12669, t12671, t12673, t12674, t12676, t12677)
}

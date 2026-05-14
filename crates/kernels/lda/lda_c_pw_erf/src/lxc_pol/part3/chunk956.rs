//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 956/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk956<F: Float>(t1289: F, t5211: F, t2076: F, t3565: F, t3660: F, t1325: F, t4632: F, t4829: F, t940: F, t1997: F, t3745: F, t3859: F, t5413: F, t197: F, t4906: F, t5417: F) -> (F, F, F, F, F, F, F, F) {
    let t12681 = 4.0 / 5.0 * t5211 * t1289;
    let t12683 = 4.0 / 15.0 * t2076 * t3565;
    let t12684 = t2076 * t3660;
    let t12685 = 8.0 / 45.0 * t12684;
    let t12689 = 16.0 / 15.0 * t1325 * t4829 * t4632 * t940;
    let t12691 = 8.0 / 15.0 * t3745 * t1997;
    let t12693 = t1325 * t3859 * t5413;
    let t12694 = 32.0 / 45.0 * t12693;
    let t12695 = t4906 * t197;
    let t12697 = t1325 * t12695 * t5417;
    (t12681, t12683, t12685, t12689, t12691, t12694, t12695, t12697)
}

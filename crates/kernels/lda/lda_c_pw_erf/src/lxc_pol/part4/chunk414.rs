//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 414/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk414<F: Float>(t148: F, t1590: F, t1131: F, t482: F, t485: F, t283: F, t732: F) -> (F, F, F) {
    let t1592 = 0.031505407223141116 * t148 * t1590;
    let t1595 = 0.003950778065781896 * t482 * t1131 * t485;
    let t1597 = t732 * t283;
    (t1592, t1595, t1597)
}

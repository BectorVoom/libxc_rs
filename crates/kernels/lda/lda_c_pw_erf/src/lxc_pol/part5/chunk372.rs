//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 372/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk372<F: Float>(t148: F, t1590: F, t1131: F, t482: F, t485: F, t283: F, t732: F) -> (F, F, F) {
    let t1592 = F::cast_from(0.031505407223141116_f64) * t148 * t1590;
    let t1595 = F::cast_from(0.003950778065781896_f64) * t482 * t1131 * t485;
    let t1597 = t732 * t283;
    (t1592, t1595, t1597)
}

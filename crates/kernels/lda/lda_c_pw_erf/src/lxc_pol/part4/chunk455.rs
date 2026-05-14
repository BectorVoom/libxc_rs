//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 455/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk455<F: Float>(t10: F, t1844: F, t431: F, t767: F, t325: F, t1697: F, t756: F) -> (F, F, F, F) {
    let t1845 = t10 * t1844;
    let t1849 = t431 * t767;
    let t1850 = t1849 * t325;
    let t1852 = t1697 * t756;
    (t1845, t1849, t1850, t1852)
}

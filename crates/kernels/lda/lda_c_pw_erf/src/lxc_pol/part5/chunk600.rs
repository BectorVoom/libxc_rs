//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 600/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk600<F: Float>(t22: F, t4048: F, t219: F, t3589: F, t1484: F, t9: F, t1210: F, t168: F, t671: F, t270: F, t2782: F, t1143: F, t466: F) -> (F, F, F, F, F, F) {
    let t4049 = t22 * t4048;
    let t4050 = t219 * t3589;
    let t4062 = t9 * t1484;
    let t4084 = t168 * t1210 * t671;
    let t4091 = F::new(0.19455129084526285) * t168 * t2782 * t270;
    let t4092 = t466 * t1143;
    (t4049, t4050, t4062, t4084, t4091, t4092)
}

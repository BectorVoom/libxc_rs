//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 570/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk570<F: Float>(t1953: F, t416: F, t1124: F, t118: F, t119: F, t120: F, t1687: F, t2061: F, t435: F, t96: F, t125: F, t917: F) -> (F, F, F, F, F) {
    let t3284 = F::cast_from(1.5156425925925925_f64) * t416 * t1953;
    let t3288 = F::cast_from(7.0_f64) / F::cast_from(27.0_f64) * t118 * t119 * t1124 * t120;
    let t3290 = F::cast_from(0.6529066666666666_f64) * t1687 * t2061;
    let t3296 = F::cast_from(1.0_f64) / t435 / t96;
    let t3309 = t125 * t917;
    (t3284, t3288, t3290, t3296, t3309)
}

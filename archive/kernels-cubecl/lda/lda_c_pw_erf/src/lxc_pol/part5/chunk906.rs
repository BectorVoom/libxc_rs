//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 906/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk906<F: Float>(t131: F, t137: F, t3337: F, t120: F, t133: F, t2869: F, t8939: F, t1552: F, t2775: F, t450: F, t147: F, t159: F, t285: F, t3165: F) -> (F, F, F, F, F, F) {
    let t9068 = t131 / t3337 / t137;
    let t9083 = F::cast_from(2.9801938271604937_f64) * t133 * t2869 * t120;
    let t9096 = t133 * t8939;
    let t9133 = t1552 * t1552;
    let t9134 = F::cast_from(1.0_f64) / t9133;
    let t9156 = t2775 * t450;
    let t9163 = F::cast_from(1.0943113336969376e-06_f64) * t3165 * t147 * t159 * t285;
    (t9068, t9083, t9096, t9134, t9156, t9163)
}

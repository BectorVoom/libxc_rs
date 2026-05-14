//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 942/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk942<F: Float>(t120: F, t133: F, t2869: F, t8939: F, t8942: F, t9021: F, t1553: F, t1724: F, t1552: F, t147: F, t159: F, t285: F, t3165: F, t1729: F, t2763: F, t164: F, t8756: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9083 = 2.9801938271604937 * t133 * t2869 * t120;
    let t9096 = t133 * t8939;
    let t9098 = t133 * t8942;
    let t9110 = t133 * t9021;
    let t9126 = t1553 * t1724;
    let t9133 = t1552 * t1552;
    let t9134 = 1.0 / t9133;
    let t9163 = 1.0943113336969376e-06 * t3165 * t147 * t159 * t285;
    let t9172 = t1729 * t2763;
    let t9178 = 0.0014238371845981686 * t8756 * t164;
    (t9083, t9096, t9098, t9110, t9126, t9134, t9163, t9172, t9178)
}

//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 816/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk816<F: Float>(t133: F, t8942: F, t8945: F, t9024: F, t9021: F, t2775: F, t452: F, t1704: F, t2765: F, t440: F, t1553: F, t1724: F, t1555: F, t1734: F, t2790: F, t1552: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9098 = t133 * t8942;
    let t9100 = t133 * t8945;
    let t9104 = t133 * t9024;
    let t9110 = t133 * t9021;
    let t9118 = t452 * t2775;
    let t9121 = t2765 * t440 * t1704;
    let t9126 = t1553 * t1724;
    let t9127 = t9126 * t1555;
    let t9130 = t2790 * t1734;
    let t9133 = t1552 * t1552;
    (t9098, t9100, t9104, t9110, t9118, t9121, t9127, t9130, t9133)
}

//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 686/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk686<F: Float>(t2557: F, t3787: F, t1325: F, t4957: F, t806: F, t4956: F, t1449: F, t2549: F, t519: F, t2553: F, t3883: F, t1475: F, t2539: F, t571: F, t2543: F, t4062: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6689 = t3787 * t2557;
    let t6690 = t1325 * t6689;
    let t6692 = t4957 * t806;
    let t6693 = t4956 * t6692;
    let t6696 = t1449 * t2549;
    let t6697 = t519 * t6696;
    let t6699 = t3883 * t2553;
    let t6700 = t519 * t6699;
    let t6702 = t1475 * t2539;
    let t6703 = t571 * t6702;
    let t6705 = t4062 * t2543;
    (t6689, t6690, t6692, t6693, t6696, t6697, t6699, t6700, t6702, t6703, t6705)
}

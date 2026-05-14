//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1359/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1359<F: Float>(t1870: F, t5639: F, t7191: F, t14679: F, t7195: F, t7199: F, t14582: F, t14585: F, t14674: F, t1568: F, t1664: F, t1871: F, t19572: F, t19575: F, t19578: F, t2594: F, t2610: F, t411: F, t5548: F, t5651: F, t6121: F, t756: F, t9096: F, t9098: F, t9110: F) -> (F,) {
    let t19739 = t1870 * t5639 * t7191;
    let t19750 = t1870 * t14679 * t7195;
    let t19753 = t1870 * t5639 * t7199;
    let t19768 = 103.4553 * t1870 * t14674 * t2594 * t1664 - t19572 + t19575 - 3.44851 * t19739 + 10.34553 * t1870 * t1871 * t6121 * t411 + 5.172765 * t1870 * t1871 * t2610 * t1568 + 13.79404 * t19750 - 6.89702 * t19753 - 20.69106 * t1870 * t5651 * t2594 * t1568 + 10.34553 * t1870 * t1871 * t756 * t5548 + t19578 + 3.5762325925925924 * t9096 - 0.7663355555555555 * t9098 + 2.2990066666666666 * t9110 + 3.5762325925925924 * t14582 - 3.065342222222222 * t14585;
    (t19768,)
}

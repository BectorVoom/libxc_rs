//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 916/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk916<F: Float>(t4606: F, t5021: F, t8141: F, t8143: F, t8146: F, t8149: F, t8155: F, t8157: F, t8159: F, t8161: F, t379: F, t386: F, t400: F, t1026: F, t1012: F) -> (F, F, F, F) {
    let t8164 = -2.8769444444444443 * t8141 + 27.618666666666666 * t8143 - 10.229135802469136 * t8146 + 8.950493827160495 * t8149 + 3.131074074074074 * t4606 + 0.0366775 * t8155 - 0.58684 * t8157 + 0.6520444444444444 * t8159 + 0.5705388888888889 * t8161 + 1.3490888888888888 * t5021;
    let t8168 = 0.5848223397455204 * t400 * t379 * t8164 * t386;
    let t8169 = t1026 * t1026;
    let t8170 = 1.0 / t8169;
    let t8171 = t1012 * t1012;
    (t8164, t8168, t8170, t8171)
}

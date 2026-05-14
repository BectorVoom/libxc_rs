//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 311/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk311<F: Float>(t1098: F, t169: F, t242: F, t299: F, t465: F, t632: F, t699: F, t145: F, t943: F, t1067: F, t703: F, t461: F, t8: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1101 = 0.14149184788746388 * t169 * t1098 * t242;
    let t1102 = t299 * t465;
    let t1104 = t169 * t1102 * t242;
    let t1108 = 0.10611888591559791 * t169 * t699 * t632;
    let t1109 = 2.0 * t145;
    let t1110 = 8.0 * t943;
    let t1111 = 6.0 * t1067;
    let t1118 = t169 * t703 * t632;
    let t1124 = 1.0 / t8 / t461;
    (t1101, t1102, t1104, t1108, t1109, t1110, t1111, t1118, t1124)
}

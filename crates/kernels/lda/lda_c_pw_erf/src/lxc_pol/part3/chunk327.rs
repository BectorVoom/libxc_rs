//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 327/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk327<F: Float>(t1098: F, t169: F, t242: F, t299: F, t465: F, t632: F, t699: F, t145: F, t943: F, t1067: F) -> (F, F, F, F, F, F, F) {
    let t1101 = F::cast_from(0.14149184788746388_f64) * t169 * t1098 * t242;
    let t1102 = t299 * t465;
    let t1104 = t169 * t1102 * t242;
    let t1108 = F::cast_from(0.10611888591559791_f64) * t169 * t699 * t632;
    let t1109 = F::new(2.0) * t145;
    let t1110 = F::new(8.0) * t943;
    let t1111 = F::new(6.0) * t1067;
    let t1112 = -t1109 + t1110 - t1111;
    (t1101, t1102, t1104, t1108, t1109, t1111, t1112)
}
